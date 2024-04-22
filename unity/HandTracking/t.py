import cv2
import mediapipe as mp

class HandGestureRecognizer:
    def __init__(self):
        self.cap = cv2.VideoCapture(0)
        self.mpHands = mp.solutions.hands
        self.hands = self.mpHands.Hands()
        self.mpDraw = mp.solutions.drawing_utils
        self.box_size = 200
        self.box_color = (0, 255, 0)  # Green color
        self.box_thickness = 2
        self.hand_gesture = None
        self.box_center = (int(self.cap.get(cv2.CAP_PROP_FRAME_WIDTH) / 2), int(self.cap.get(cv2.CAP_PROP_FRAME_HEIGHT) / 2))
        self.hand_in_box = False
        self.velocity_x = 0
        self.velocity_y = 0
        self.friction = 0.95
        self.deceleration = 0.1

    def run(self):
        while True:
            success, img = self.cap.read()
            img = cv2.flip(img, 1)
            imgRGB = cv2.cvtColor(img, cv2.COLOR_BGR2RGB)
            results = self.hands.process(imgRGB)

            if results.multi_hand_landmarks:
                for handLms in results.multi_hand_landmarks:
                    self.mpDraw.draw_landmarks(img, handLms, self.mpHands.HAND_CONNECTIONS)

                    # Classify hand gesture
                    self.hand_gesture = self.classify_hand_gesture(handLms)
                    if self.hand_gesture == "Pinch":
                        self.hand_in_box = self.is_hand_in_box(handLms)
                        if self.hand_in_box:
                            self.update_box_center(handLms)
                    else:
                        self.hand_in_box = False

            self.update_box_position()
            self.draw_box_overlay(img)
            cv2.imshow("Image", img)
            if cv2.waitKey(1) == ord('q'):
                break

        self.cap.release()
        cv2.destroyAllWindows()

    def classify_hand_gesture(self, hand_landmarks):
        thumb_tip = hand_landmarks.landmark[self.mpHands.HandLandmark.THUMB_TIP]
        index_finger_tip = hand_landmarks.landmark[self.mpHands.HandLandmark.INDEX_FINGER_TIP]
        dist_thumb_index = abs(thumb_tip.x - index_finger_tip.x) + abs(thumb_tip.y - index_finger_tip.y)

        if dist_thumb_index < 0.06:
            return "Pinch"
        else:
            return "Open"

    def update_box_center(self, hand_landmarks):
        x_coords = [lm.x for lm in hand_landmarks.landmark]
        y_coords = [lm.y for lm in hand_landmarks.landmark]
        x_min, x_max = min(x_coords), max(x_coords)
        y_min, y_max = min(y_coords), max(y_coords)
        hand_center = ((x_min + x_max) / 2, (y_min + y_max) / 2)

        self.box_center = (int(hand_center[0] * self.cap.get(cv2.CAP_PROP_FRAME_WIDTH)), int(hand_center[1] * self.cap.get(cv2.CAP_PROP_FRAME_HEIGHT)))

    def is_hand_in_box(self, hand_landmarks):
        x1 = self.box_center[0] - self.box_size // 2
        y1 = self.box_center[1] - self.box_size // 2
        x2 = self.box_center[0] + self.box_size // 2
        y2 = self.box_center[1] + self.box_size // 2

        hand_x = hand_landmarks.landmark[self.mpHands.HandLandmark.INDEX_FINGER_TIP].x * self.cap.get(cv2.CAP_PROP_FRAME_WIDTH)
        hand_y = hand_landmarks.landmark[self.mpHands.HandLandmark.INDEX_FINGER_TIP].y * self.cap.get(cv2.CAP_PROP_FRAME_HEIGHT)

        if x1 < hand_x < x2 and y1 < hand_y < y2:
            return True
        else:
            return False

    def update_box_position(self):
        if not self.hand_in_box:
            self.velocity_x *= self.friction
            self.velocity_y *= self.friction
        else:
            self.velocity_x = 0
            self.velocity_y = 0

        self.box_center = (
            self.box_center[0] + int(self.velocity_x),
            self.box_center[1] + int(self.velocity_y)
        )

    def draw_box_overlay(self, img):
        x1 = self.box_center[0] - self.box_size // 2
        y1 = self.box_center[1] - self.box_size // 2
        x2 = self.box_center[0] + self.box_size // 2
        y2 = self.box_center[1] + self.box_size // 2
        cv2.rectangle(img, (x1, y1), (x2, y2), self.box_color, self.box_thickness)

hand_recognizer = HandGestureRecognizer()
hand_recognizer.run()