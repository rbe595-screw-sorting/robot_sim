#!/usr/bin/env python3
"""
Screw detection + localisation node.
 
Pipeline
  1. Detect screws in the colour image   (classical CV, YOLO, or hybrid)
  2. Back-project pixel -> 3D in `world_frame` (table-plane ray intersection,
     aligned depth image, or fixed depth)
  3. Track detections over several frames (stable IDs, smoothing, rejects flicker)
  4. Publish a TF frame per confirmed screw (screw_<id>), a PoseArray,
     RViz markers, and a debug image.
"""
 
import math
import os
 
import cv2
import numpy as np
import rclpy
import tf2_ros
from cv_bridge import CvBridge
from rcl_interfaces.msg import SetParametersResult
from geometry_msgs.msg import Point, Pose, PoseArray, PointStamped, TransformStamped
from rclpy.duration import Duration
from rclpy.node import Node
from rclpy.qos import qos_profile_sensor_data
from rclpy.time import Time
from sensor_msgs.msg import CameraInfo, Image
from visualization_msgs.msg import Marker, MarkerArray
 
 
def quat_to_rot(q):
    x, y, z, w = q.x, q.y, q.z, q.w
    return np.array([
        [1 - 2 * (y * y + z * z), 2 * (x * y - z * w),     2 * (x * z + y * w)],
        [2 * (x * y + z * w),     1 - 2 * (x * x + z * z), 2 * (y * z - x * w)],
        [2 * (x * z - y * w),     2 * (y * z + x * w),     1 - 2 * (x * x + y * y)],
    ])
 
 
class Detection(Node):
    def __init__(self):
        super().__init__('cv_detections')
 
        # ---------------- parameters ----------------
        # general
        self.declare_parameter('detector', 'classical')      # classical | yolo | hybrid
        self.declare_parameter('rate_hz', 5.0)
        self.declare_parameter('world_frame', 'base_link')
        self.declare_parameter('screw_frame_prefix', 'screw')
 
        # classical CV
        self.declare_parameter('threshold', 0)               # 0 -> Otsu (automatic)
        self.declare_parameter('polarities', ['dark', 'light'])
        self.declare_parameter('min_length', 5)             # px, long side of min-area rect
        self.declare_parameter('max_length', 100)
        self.declare_parameter('min_area', 40)              # px^2
        self.declare_parameter('max_area_frac', 0.001)        # reject blobs bigger than this fraction of image
        self.declare_parameter('min_aspect', 1.2)            # set ~1.5-2.0 if screws are lying on their side
        self.declare_parameter('min_solidity', 0.4)         # area / convex hull area
        self.declare_parameter('reject_border', True)
        self.declare_parameter('min_width', 5)               # px, short side of min-area rect
        self.declare_parameter('max_width', 100)
        self.declare_parameter('workspace', [-5.0, 5.0, -5.0, 5.0])  # [xmin, xmax, ymin, ymax] in world_frame (m)
        self.declare_parameter('debug_metrics', True)        # print per-blob numbers on /debug/image_proc
        self.declare_parameter('mask_methods', ['global', 'edges'])  # any of: global local edges
        self.declare_parameter('mask_close', 7)              # px; raise (15-25) to re-join fragmented shiny screws
        self.declare_parameter('local_thresh', 25)           # min brightness difference vs local background
        self.declare_parameter('local_bg_kernel', 61)        # px; must be wider than the screw's short side
        self.declare_parameter('canny_low', 40)
        self.declare_parameter('canny_high', 140)
        self.declare_parameter('duplicate_iou', 0.50)
        self.declare_parameter('duplicate_distance', 20)
 
        # YOLO
        self.declare_parameter('model_path', 'weights/best.pt')
        self.declare_parameter('confidence', 0.5)
        self.declare_parameter('inference_dev', 'amd')       # amd | nvidia | cpu
 
        # 3D position
        self.declare_parameter('position_mode', 'plane')     # plane | depth | fixed
        self.declare_parameter('plane_z', 0.0)               # z of screw centre in world_frame (plane mode)
        self.declare_parameter('fixed_depth', 0.1142)        # m (fixed mode)
        self.declare_parameter('color_topic', '/camera/camera/color/image_raw')
        self.declare_parameter('info_topic', '/camera/camera/color/camera_info')
        self.declare_parameter('depth_topic', '/camera/camera/aligned_depth_to_color/image_raw')
 
        # tracking
        self.declare_parameter('track_match_dist', 0.05)     # m
        self.declare_parameter('track_alpha', 0.3)           # EMA weight of new observation
        self.declare_parameter('min_hits', 10)
        self.declare_parameter('max_misses', 5)
 
        # visualisation
        self.declare_parameter('marker_size', 0.02)
 
        p = lambda n: self.get_parameter(n).value
        self.detector = p('detector')
        self.world_frame = p('world_frame')
        self.frame_prefix = p('screw_frame_prefix')
        self.threshold = p('threshold')
        self.polarities = list(p('polarities'))
        self.min_length = p('min_length')
        self.max_length = p('max_length')
        self.min_area = p('min_area')
        self.max_area_frac = p('max_area_frac')
        self.min_aspect = p('min_aspect')
        self.min_solidity = p('min_solidity')
        self.reject_border = p('reject_border')
        self.mask_methods = list(p('mask_methods'))
        self.min_width = p('min_width')
        self.max_width = p('max_width')
        self.workspace = list(p('workspace'))
        self.debug_metrics = p('debug_metrics')
        self.mask_close = p('mask_close')
        self.local_thresh = p('local_thresh')
        self.local_bg_kernel = p('local_bg_kernel')
        self.canny_low = p('canny_low')
        self.canny_high = p('canny_high')
        self.duplicate_iou = p('duplicate_iou')
        self.duplicate_distance = p('duplicate_distance')
        self.conf_thres = p('confidence')
        self.position_mode = p('position_mode')
        self.plane_z = p('plane_z')
        self.fixed_depth = p('fixed_depth')
        self.match_dist = p('track_match_dist')
        self.alpha = p('track_alpha')
        self.min_hits = p('min_hits')
        self.max_misses = p('max_misses')
        self.marker_size = p('marker_size')
 
        self.add_on_set_parameters_callback(self.on_param_change)
 
        # ---------------- state ----------------
        self.br = CvBridge()
        self.img_msg = None
        self.depth_msg = None
        self.depth_np = None
        self.K = None
        self.D = None
        self.last_stamp = None
        self.tracks = []
        self.next_id = 0
        self.reject_counts = {}
        self.model = None
        self.device = 'cpu'
 
        # ---------------- YOLO (optional) ----------------
        if self.detector in ('yolo', 'hybrid'):
            self._load_yolo(p('model_path'), p('inference_dev'))
 
        # ---------------- ROS I/O ----------------
        self.tf_buffer = tf2_ros.Buffer()
        self.tf_listener = tf2_ros.TransformListener(self.tf_buffer, self)
        self.tf_broadcaster = tf2_ros.TransformBroadcaster(self)
 
        self.create_subscription(Image, p('color_topic'), self.on_image, qos_profile_sensor_data)
        self.create_subscription(CameraInfo, p('info_topic'), self.on_info, qos_profile_sensor_data)
        if self.position_mode == 'depth':
            self.create_subscription(Image, p('depth_topic'), self.on_depth, qos_profile_sensor_data)
 
        self.screw_world_pts = self.create_publisher(PointStamped, '/screw_location', 10)
        self.pose_pub = self.create_publisher(PoseArray, '/screw_poses', 10)
        self.debug_cv = self.create_publisher(Image, '/debug/image_proc', 10)
        self.debug_yolo = self.create_publisher(Image, '/debug/yolo', 10)
        self.debug_mask = self.create_publisher(Image, '/debug/mask', 10)
        # raw, un-smoothed per-frame detections (used by the grasp executor)
        self.obs_pub = self.create_publisher(PoseArray, '/screw_observations', 10)
        self.world_pt_debug = self.create_publisher(MarkerArray, '/debug/world_pts', 10)
 
        self.create_timer(1.0 / max(p('rate_hz'), 0.1), self.process_image)
        self.get_logger().info(
            f"detector={self.detector}, position_mode={self.position_mode}, world_frame={self.world_frame}")
 
    # =====================================================================
    # setup helpers / callbacks
    # =====================================================================
    def _load_yolo(self, model_path, dev):
        try:
            from ultralytics import YOLO
            if dev == 'amd':
                os.environ["HIP_VISIBLE_DEVICES"] = "0"
                self.device = "cuda:0"
            elif dev == 'nvidia':
                self.device = "cuda"
            elif dev == 'cpu':
                self.device = "cpu"
            else:
                raise ValueError(f"inference_dev '{dev}' must be amd, nvidia or cpu")
            self.model = YOLO(model_path)
            self.model.to(self.device)
            self.get_logger().info(f"Loaded YOLO model {model_path} on {self.device}")
        except Exception as exc:
            self.get_logger().error(f"YOLO load failed ({exc}); falling back to classical detector")
            self.model = None
            self.detector = 'classical'
 
    # parameter name -> attribute name, for live tuning with `ros2 param set`
    RUNTIME_PARAMS = {
        'threshold': 'threshold', 'polarities': 'polarities',
        'min_length': 'min_length', 'max_length': 'max_length',
        'min_area': 'min_area', 'max_area_frac': 'max_area_frac',
        'min_aspect': 'min_aspect', 'min_solidity': 'min_solidity',
        'reject_border': 'reject_border',
        'min_width': 'min_width', 'max_width': 'max_width',
        'workspace': 'workspace', 'debug_metrics': 'debug_metrics',
        'mask_methods': 'mask_methods', 'mask_close': 'mask_close',
        'local_thresh': 'local_thresh', 'local_bg_kernel': 'local_bg_kernel',
        'canny_low': 'canny_low', 'canny_high': 'canny_high',
        'duplicate_iou': 'duplicate_iou', 'duplicate_distance': 'duplicate_distance',
        'confidence': 'conf_thres',
        'plane_z': 'plane_z', 'fixed_depth': 'fixed_depth',
        'track_match_dist': 'match_dist', 'track_alpha': 'alpha',
        'min_hits': 'min_hits', 'max_misses': 'max_misses',
        'marker_size': 'marker_size',
    }
 
    def on_param_change(self, params):
        for prm in params:
            attr = self.RUNTIME_PARAMS.get(prm.name)
            if attr is None:
                return SetParametersResult(
                    successful=False, reason=f"'{prm.name}' can't be changed at runtime (restart the node)")
            setattr(self, attr, prm.value)
            self.get_logger().info(f"{prm.name} = {prm.value}")
        self.tracks = []   # old tracks were built with the old settings
        return SetParametersResult(successful=True)
 
    def on_image(self, msg: Image):
        self.img_msg = msg   # converted lazily in the timer (saves CPU)
 
    def on_depth(self, msg: Image):
        self.depth_msg = msg
 
    def on_info(self, msg: CameraInfo):
        self.K = np.asarray(msg.k, dtype=np.float64).reshape(3, 3)
        self.D = np.asarray(msg.d, dtype=np.float64) if len(msg.d) else np.zeros(5)
 
    # =====================================================================
    # 2D detection
    # =====================================================================
    @staticmethod
    def intersection_over_union(a, b):
        ax, ay, aw, ah = a
        bx, by, bw, bh = b
        ix = max(0, min(ax + aw, bx + bw) - max(ax, bx))
        iy = max(0, min(ay + ah, by + bh) - max(ay, by))
        inter = ix * iy
        union = aw * ah + bw * bh - inter
        return inter / union if union else 0.0
 
    def is_duplicate(self, a, b):
        iou = self.intersection_over_union(a['bbox'], b['bbox'])
        dist = np.linalg.norm(np.array(a['centroid']) - np.array(b['centroid']))
        return iou > self.duplicate_iou or dist < self.duplicate_distance
 
    def remove_duplicates(self, dets):
        unique = []
        for d in sorted(dets, key=lambda d: -d['area']):   # keep the larger blob
            if not any(self.is_duplicate(d, u) for u in unique):
                unique.append(d)
        return unique
 
    def _clean(self, mask):
        k = max(1, int(self.mask_close))
        k_close = cv2.getStructuringElement(cv2.MORPH_ELLIPSE, (k, k))
        k_open = cv2.getStructuringElement(cv2.MORPH_ELLIPSE, (3, 3))
        mask = cv2.morphologyEx(mask, cv2.MORPH_CLOSE, k_close)
        return cv2.morphologyEx(mask, cv2.MORPH_OPEN, k_open)
 
    def create_mask(self, gray, polarity):
        """Global threshold (Otsu if threshold<=0)."""
        flag = cv2.THRESH_BINARY_INV if polarity == 'dark' else cv2.THRESH_BINARY
        if self.threshold <= 0:
            _, mask = cv2.threshold(gray, 0, 255, flag + cv2.THRESH_OTSU)
        else:
            _, mask = cv2.threshold(gray, self.threshold, 255, flag)
        return self._clean(mask)
 
    def local_mask(self, gray):
        """Anything that differs from its local background, bright OR dark.
        Works for mid-gray / shiny objects that a global threshold misses."""
        h, w = gray.shape
        s = 4   # estimate the background at 1/4 resolution (fast)
        small = cv2.resize(gray, (max(1, w // s), max(1, h // s)), interpolation=cv2.INTER_AREA)
        k = max(3, (int(self.local_bg_kernel) // s) | 1)
        bg = cv2.resize(cv2.medianBlur(small, k), (w, h), interpolation=cv2.INTER_LINEAR)
        _, mask = cv2.threshold(cv2.absdiff(gray, bg), self.local_thresh, 255, cv2.THRESH_BINARY)
        return self._clean(mask)
 
    def edge_mask(self, gray):
        """Fill closed Canny outlines. Metal has strong edges even when its brightness matches the table."""
        edges = cv2.Canny(gray, self.canny_low, self.canny_high)
        k = cv2.getStructuringElement(cv2.MORPH_ELLIPSE, (5, 5))
        edges = cv2.morphologyEx(edges, cv2.MORPH_CLOSE, k, iterations=2)
        cnts, _ = cv2.findContours(edges, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
        mask = np.zeros_like(edges)
        cv2.drawContours(mask, cnts, -1, 255, cv2.FILLED)
        return mask
 
    def _rej(self, reason):
        self.reject_counts[reason] = self.reject_counts.get(reason, 0) + 1
        return None
 
    def evaluate_contour(self, contour, shape):
        """Shape-filter a contour; return a candidate dict or None."""
        h_img, w_img = shape[:2]
        area = cv2.contourArea(contour)
        if area < self.min_area:
            return self._rej('min_area')
        if area > self.max_area_frac * h_img * w_img:
            return self._rej('max_area_frac')
 
        rect = cv2.minAreaRect(contour)
        (_, _), (rw, rh), _ = rect
        length, width = max(rw, rh), min(rw, rh)
        if width < 1:
            return self._rej('degenerate')
        if length < self.min_length:
            return self._rej('min_length')
        if length > self.max_length:
            return self._rej('max_length')
        if width < self.min_width:
            return self._rej('min_width')
        if width > self.max_width:
            return self._rej('max_width')
        if length / width < self.min_aspect:
            return self._rej('min_aspect')
 
        hull_area = cv2.contourArea(cv2.convexHull(contour))
        if hull_area <= 0 or area / hull_area < self.min_solidity:
            return self._rej('min_solidity')
 
        x, y, w, h = cv2.boundingRect(contour)
        if self.reject_border and (x <= 1 or y <= 1 or x + w >= w_img - 1 or y + h >= h_img - 1):
            return self._rej('border')
 
        m = cv2.moments(contour)
        if m['m00'] == 0:
            return self._rej('degenerate')
        cx, cy = m['m10'] / m['m00'], m['m01'] / m['m00']
 
        pts = cv2.boxPoints(rect)
        e0, e1 = pts[1] - pts[0], pts[2] - pts[1]
        e = e0 if np.linalg.norm(e0) >= np.linalg.norm(e1) else e1
        axis = e / (np.linalg.norm(e) + 1e-9)
 
        return {'centroid': (cx, cy), 'axis': axis, 'bbox': (x, y, w, h),
                'contour': contour, 'area': area, 'conf': 1.0,
                'length': length, 'width': width, 'solidity': area / hull_area}
 
    def detect_classical(self, bgr):
        gray = cv2.GaussianBlur(cv2.cvtColor(bgr, cv2.COLOR_BGR2GRAY), (5, 5), 0)
 
        masks = []
        if 'global' in self.mask_methods:
            for polarity in self.polarities:
                masks.append((f'global-{polarity}', self.create_mask(gray, polarity)))
        if 'local' in self.mask_methods:
            masks.append(('local', self.local_mask(gray)))
        if 'edges' in self.mask_methods:
            masks.append(('edges', self.edge_mask(gray)))
 
        self.reject_counts = {}
        cands = []
        combined = np.zeros_like(gray)
        for name, mask in masks:
            combined = cv2.bitwise_or(combined, mask)
            contours, _ = cv2.findContours(mask, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
            for c in contours:
                cand = self.evaluate_contour(c, bgr.shape)
                if cand is not None:
                    cand['source'] = name
                    cands.append(cand)
 
        self.debug_mask.publish(self.br.cv2_to_imgmsg(combined, encoding='mono8'))
        # if self.reject_counts:
        #     self.get_logger().info(f"rejected blobs by filter: {self.reject_counts}",
        #                            throttle_duration_sec=2.0)
        return self.remove_duplicates(cands)
 
    def detect_yolo(self, bgr):
        result = self.model(bgr, conf=self.conf_thres, device=self.device, verbose=False)[0]
        try:
            self.debug_yolo.publish(self.br.cv2_to_imgmsg(result.plot(), encoding='bgr8'))
        except Exception:
            pass
        dets = []
        xyxy = result.boxes.xyxy.cpu().numpy()
        confs = result.boxes.conf.cpu().numpy()
        for (x1, y1, x2, y2), conf in zip(xyxy, confs):
            w, h = x2 - x1, y2 - y1
            dets.append({
                'centroid': ((x1 + x2) / 2, (y1 + y2) / 2),
                'axis': np.array([1.0, 0.0]) if w >= h else np.array([0.0, 1.0]),  # rough
                'bbox': (int(x1), int(y1), int(w), int(h)),
                'contour': None, 'area': w * h, 'conf': float(conf)})
        return dets
 
    def detect(self, bgr):
        if self.detector == 'classical' or self.model is None:
            return self.detect_classical(bgr)
 
        yolo = self.detect_yolo(bgr)
        if self.detector == 'yolo':
            return yolo
 
        # hybrid: YOLO decides *what* is a screw, classical CV refines centroid + orientation
        classical = self.detect_classical(bgr)
        out = []
        for y in yolo:
            x, yy, w, h = y['bbox']
            inside = [c for c in classical
                      if x <= c['centroid'][0] <= x + w and yy <= c['centroid'][1] <= yy + h]
            if inside:
                best = max(inside, key=lambda c: c['area'])
                best['conf'] = y['conf']
                out.append(best)
            else:
                out.append(y)
        return out
 
    # =====================================================================
    # 2D -> 3D
    # =====================================================================
    def pixel_ray(self, u, v):
        """Unit-depth ray (z=1) in the camera optical frame, distortion-corrected."""
        n = cv2.undistortPoints(np.array([[[u, v]]], dtype=np.float64), self.K, self.D)
        return np.array([n[0, 0, 0], n[0, 0, 1], 1.0])
 
    def sample_depth(self, u, v, r=3):
        if self.depth_np is None:
            return None
        h, w = self.depth_np.shape[:2]
        u, v = int(round(u)), int(round(v))
        patch = self.depth_np[max(0, v - r):min(h, v + r + 1), max(0, u - r):min(w, u + r + 1)]
        patch = patch[np.isfinite(patch) & (patch > 0)]
        return float(np.median(patch)) if patch.size else None
 
    def pixel_to_world(self, u, v, R, t, z=None):
        """
        Returns (point_in_world (3,), camera_depth) or None.
        R, t: transform camera optical frame -> world frame.
        """
        ray = self.pixel_ray(u, v)
 
        if self.position_mode == 'plane' and z is None:
            # intersect camera ray with horizontal plane z = plane_z in world frame
            d = R @ ray
            if abs(d[2]) < 1e-6:
                return None
            s = (self.plane_z - t[2]) / d[2]
            if s <= 0:
                return None
            return t + s * d, s
 
        if z is None:
            z = self.sample_depth(u, v) if self.position_mode == 'depth' else self.fixed_depth
        if z is None or not np.isfinite(z) or z <= 0:
            return None
        return R @ (ray * z) + t, z
 
    def lookup_cam_tf(self, frame, stamp):
        try:
            tf = self.tf_buffer.lookup_transform(
                self.world_frame, frame, Time.from_msg(stamp), timeout=Duration(seconds=0.0))
        except tf2_ros.TransformException:
            try:   # fall back to latest (non-blocking!)
                tf = self.tf_buffer.lookup_transform(self.world_frame, frame, Time())
            except tf2_ros.TransformException as exc:
                self.get_logger().warn(f"TF {self.world_frame}<-{frame} unavailable: {exc}",
                                       throttle_duration_sec=2.0)
                return None
        tr = tf.transform.translation
        return quat_to_rot(tf.transform.rotation), np.array([tr.x, tr.y, tr.z])
 
    # =====================================================================
    # tracking
    # =====================================================================
    def update_tracks(self, observations):
        for tr in self.tracks:
            tr['matched'] = False
 
        ids = []
        for pos, yaw in observations:
            best, best_d = None, self.match_dist
            for tr in self.tracks:
                if tr['matched']:
                    continue
                d = np.linalg.norm(tr['pos'] - pos)
                if d < best_d:
                    best, best_d = tr, d
            if best is None:
                self.tracks.append({'id': self.next_id, 'pos': pos.copy(),
                                    'c': math.cos(2 * yaw), 's': math.sin(2 * yaw),
                                    'hits': 1, 'misses': 0, 'matched': True})
                ids.append(self.next_id)
                self.next_id += 1
            else:
                a = self.alpha
                best['pos'] = (1 - a) * best['pos'] + a * pos
                # screws are symmetric under 180deg -> smooth the doubled angle
                best['c'] = (1 - a) * best['c'] + a * math.cos(2 * yaw)
                best['s'] = (1 - a) * best['s'] + a * math.sin(2 * yaw)
                best['hits'] += 1
                best['misses'] = 0
                best['matched'] = True
                ids.append(best['id'])
                # if best['hits'] == self.min_hits:
                #     self.get_logger().info(
                #         f"screw #{best['id']} confirmed at "
                #         f"({best['pos'][0]:.3f}, {best['pos'][1]:.3f}) -> TF '{self.frame_prefix}_{best['id']}'")
 
        for tr in self.tracks:
            if not tr['matched']:
                tr['misses'] += 1
        # for t in self.tracks:
            # if t['misses'] > self.max_misses and t['hits'] >= self.min_hits:
            #     self.get_logger().info(f"screw #{t['id']} lost (not seen for {t['misses']} frames)")
        self.tracks = [t for t in self.tracks if t['misses'] <= self.max_misses]
        return ids
 
    @staticmethod
    def track_yaw(tr):
        return 0.5 * math.atan2(tr['s'], tr['c'])
 
    # =====================================================================
    # publishing
    # =====================================================================
    def publish_tracks(self):
        now = self.get_clock().now().to_msg()
        confirmed = [t for t in self.tracks if t['hits'] >= self.min_hits]
 
        poses = PoseArray()
        poses.header.frame_id = self.world_frame
        poses.header.stamp = now
        markers = MarkerArray()
        transforms = []
 
        for tr in confirmed:
            yaw = self.track_yaw(tr)
            qz, qw = math.sin(yaw / 2), math.cos(yaw / 2)
            x, y, z = (float(v) for v in tr['pos'])
 
            pose = Pose()
            pose.position.x, pose.position.y, pose.position.z = x, y, z
            pose.orientation.z, pose.orientation.w = qz, qw
            poses.poses.append(pose)
 
            tf = TransformStamped()
            tf.header.stamp = now
            tf.header.frame_id = self.world_frame
            tf.child_frame_id = f"{self.frame_prefix}_{tr['id']}"
            tf.transform.translation.x, tf.transform.translation.y, tf.transform.translation.z = x, y, z
            tf.transform.rotation.z, tf.transform.rotation.w = qz, qw
            transforms.append(tf)
 
            pt = PointStamped()
            pt.header.stamp = now
            pt.header.frame_id = self.world_frame
            pt.point.x, pt.point.y, pt.point.z = x, y, z
            self.screw_world_pts.publish(pt)
 
            m = Marker()
            m.header.frame_id = self.world_frame
            m.header.stamp = now
            m.ns = 'screws'
            m.id = tr['id']
            m.type = Marker.ARROW
            m.action = Marker.ADD
            m.pose = pose
            m.scale.x = self.marker_size * 3
            m.scale.y = self.marker_size * 0.5
            m.scale.z = self.marker_size * 0.5
            m.color.g, m.color.a = 1.0, 1.0
            m.lifetime = Duration(seconds=1.0).to_msg()
            markers.markers.append(m)
 
        if transforms:
            self.tf_broadcaster.sendTransform(transforms)
        self.pose_pub.publish(poses)
        self.world_pt_debug.publish(markers)
 
    # =====================================================================
    # main loop
    # =====================================================================
    def process_image(self):
        if self.img_msg is None or self.K is None:
            self.get_logger().warn("no image message or camera info yet", throttle_duration_sec=2.0)
            return
 
        msg = self.img_msg
        stamp_key = (msg.header.stamp.sec, msg.header.stamp.nanosec)
        if stamp_key == self.last_stamp:
            return   # nothing new
        self.last_stamp = stamp_key
 
        bgr = self.br.imgmsg_to_cv2(msg, desired_encoding='bgr8')
 
        if self.position_mode == 'depth':
            if self.depth_msg is None:
                self.get_logger().warn("waiting for depth image", throttle_duration_sec=2.0)
                return
            depth = self.br.imgmsg_to_cv2(self.depth_msg, desired_encoding='passthrough')
            self.depth_np = depth.astype(np.float32) * (0.001 if depth.dtype == np.uint16 else 1.0)
 
        cam_tf = self.lookup_cam_tf(msg.header.frame_id, msg.header.stamp)
        if cam_tf is None:
            return
        R, t = cam_tf
 
        candidates = self.detect(bgr)
        overlay = bgr.copy()
        observations = []
        label_px = []   # pixel centroid per observation, to draw track IDs after tracking
 
        for c in candidates:
            u, v = c['centroid']
            res = self.pixel_to_world(u, v, R, t)
            if res is None:
                continue
            p_world, z = res
            xmin, xmax, ymin, ymax = self.workspace
            if not (xmin <= p_world[0] <= xmax and ymin <= p_world[1] <= ymax):
                continue   # outside the allowed workspace
 
            # orientation: project a second point along the screw's long axis
            yaw = 0.0
            u2, v2 = np.array([u, v]) + c['axis'] * 20.0
            res2 = self.pixel_to_world(u2, v2, R, t, z=None if self.position_mode == 'plane' else z)
            if res2 is not None:
                d = res2[0] - p_world
                yaw = math.atan2(d[1], d[0])
 
            observations.append((p_world, yaw))
            label_px.append((int(u), int(v)))
 
            # debug overlay
            if c['contour'] is not None:
                cv2.drawContours(overlay, [c['contour']], -1, (255, 255, 0), 1)
            cv2.drawMarker(overlay, (int(u), int(v)), (0, 255, 0), cv2.MARKER_CROSS, 14, 2)
            cv2.line(overlay, (int(u), int(v)), (int(u2), int(v2)), (0, 0, 255), 2)
            cv2.putText(overlay, f"{p_world[0]:.3f},{p_world[1]:.3f},{p_world[2]:.3f}",
                        (int(u) + 8, int(v) - 8), cv2.FONT_HERSHEY_SIMPLEX, 0.45, (0, 255, 0), 1)
            if self.debug_metrics and 'length' in c:
                info = (f"{c.get('source', '?')} A={c['area']:.0f} L={c['length']:.0f} "
                        f"W={c['width']:.0f} L/W={c['length'] / c['width']:.1f} S={c['solidity']:.2f}")
                cv2.putText(overlay, info, (int(u) + 8, int(v) + 12),
                            cv2.FONT_HERSHEY_SIMPLEX, 0.4, (0, 200, 255), 1)
 
        obs_msg = PoseArray()
        obs_msg.header.frame_id = self.world_frame
        obs_msg.header.stamp = self.get_clock().now().to_msg()
        for pos, yaw in observations:
            o = Pose()
            o.position.x, o.position.y, o.position.z = (float(v) for v in pos)
            o.orientation.z, o.orientation.w = math.sin(yaw / 2), math.cos(yaw / 2)
            obs_msg.poses.append(o)
        self.obs_pub.publish(obs_msg)
 
        ids = self.update_tracks(observations)
        self.publish_tracks()
 
        # tag each detection with its track ID: green = confirmed (TF is published), orange = tentative
        by_id = {t['id']: t for t in self.tracks}
        for (u, v), tid in zip(label_px, ids):
            tr = by_id.get(tid)
            confirmed = tr is not None and tr['hits'] >= self.min_hits
            color = (0, 255, 0) if confirmed else (0, 165, 255)
            text = f"#{tid}" if confirmed else f"#{tid} ({tr['hits'] if tr else 0}/{self.min_hits})"
            cv2.putText(overlay, text, (u + 8, v - 28), cv2.FONT_HERSHEY_SIMPLEX, 0.8, (0, 0, 0), 4)
            cv2.putText(overlay, text, (u + 8, v - 28), cv2.FONT_HERSHEY_SIMPLEX, 0.8, color, 2)
 
        self.debug_cv.publish(self.br.cv2_to_imgmsg(overlay, encoding='bgr8'))
 
 
def main(args=None):
    rclpy.init(args=args)
    node = Detection()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    node.destroy_node()
    rclpy.shutdown()
 
 
if __name__ == '__main__':
    main()
 
