import cv2
import json
import os

# Load the pre-trained Haar Cascade for face detection
cascade = cv2.CascadeClassifier('haarcascade_frontalface_default.xml')

def detect_faces(frame):
    gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
    faces = cascade.detectMultiScale(gray, scaleFactor=1.1, minNeighbors=5)
    return faces

def main():
    cap = cv2.VideoCapture(0)  # Capture video from the default camera

    if not cap.isOpened():
        print("Error: Camera not accessible.")
        return

    # Create an initial empty face_coords.json file
    with open("face_coords.json", "w") as f:
        json.dump([], f)  # Use an empty list to initialize the file

    while True:
        ret, frame = cap.read()
        if not ret:
            break

        faces = detect_faces(frame)

        # Draw rectangles around detected faces
        for (x, y, w, h) in faces:
            cv2.rectangle(frame, (x, y), (x + w, y + h), (0, 255, 0), 2)

        # Save coordinates to face_coords.json
        coords = [{"x": int(x), "y": int(y), "w": int(w), "h": int(h)} for (x, y, w, h) in faces]
        with open("face_coords.json", "w") as f:
            json.dump(coords, f)

        # Display the video feed
        cv2.imshow("Face Detection", frame)

        if cv2.waitKey(1) & 0xFF == ord('q'):
            break

    cap.release()
    cv2.destroyAllWindows()

if __name__ == "__main__":
    main()
