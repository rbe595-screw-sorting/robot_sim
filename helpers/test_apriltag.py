import cv2

# Load the saved wrist camera image
img = cv2.imread("/tmp/apriltag_test.png")

if img is None:
    print("Could not load image!")
    exit()

print("OpenCV version:", cv2.__version__)

# Convert to grayscale
gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)

# Load AprilTag 16h5 dictionary
dictionary = cv2.aruco.getPredefinedDictionary(
    cv2.aruco.DICT_APRILTAG_16h5
)

# Create detector
parameters = cv2.aruco.DetectorParameters()
detector = cv2.aruco.ArucoDetector(dictionary, parameters)

# Detect markers
corners, ids, rejected = detector.detectMarkers(gray)

print("Detected IDs:", ids)
print("Rejected candidates:", len(rejected))

# Draw detections
if ids is not None:
    cv2.aruco.drawDetectedMarkers(img, corners, ids)

cv2.imwrite("/tmp/apriltag_result.png", img)
print("Saved annotated image to /tmp/apriltag_result.png")