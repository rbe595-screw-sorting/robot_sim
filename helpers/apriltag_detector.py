import cv2
import numpy as np


class AprilTagDetector:

    def __init__(
        self,
        tag_family=cv2.aruco.DICT_APRILTAG_16h5,
        tolerance=10
    ):

        self.dictionary = cv2.aruco.getPredefinedDictionary(tag_family)

        self.parameters = cv2.aruco.DetectorParameters_create()

        self.tolerance = tolerance

    def detect(self, image):

        #make it gray!!
        gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)

        #checking img
        corners, ids, rejected = cv2.aruco.detectMarkers(
            gray,
            self.dictionary,
            parameters=self.parameters
        )

        output = image.copy()
        detected_tags = []

        if ids is not None:

            cv2.aruco.drawDetectedMarkers(output, corners, ids)

            for i, tag_id in enumerate(ids.flatten()):

                tag_corners = corners[i][0]

                #centroid
                centroid_x = int(np.mean(tag_corners[:, 0]))
                centroid_y = int(np.mean(tag_corners[:, 1]))

                #sh ave
                detected_tags.append({
                    "id": int(tag_id),
                    "centroid": (centroid_x, centroid_y)
                })

                #annotate image!
                cv2.circle(
                    output,
                    (centroid_x, centroid_y),
                    5,
                    (0, 0, 255),
                    -1
                )

                cv2.putText(
                    output,
                    f"ID: {tag_id}",
                    (centroid_x, centroid_y - 10),
                    cv2.FONT_HERSHEY_SIMPLEX,
                    0.5,
                    (0, 255, 0),
                    2
                )

        return detected_tags, output