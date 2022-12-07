import matplotlib.pyplot as plt
from pathlib import Path
import numpy as np
import websockets
import asyncio
import json
import cv2 
import os 


def truncate_frame(frame: np.ndarray, width: int, height: int) -> np.ndarray:
    # convert to black/white (with 1 value)
    frame = frame[:, :, 0]

    # reduce dimensions to fit width/height
    h, w = frame.shape
    window_w, window_h = w / width, h / height

    # for x in range(width):
    #     for y in range(height):
    #         array = frame[round(y * window_h):round(y * window_h + window_h),
    #                       round(x * window_w):round(x * window_w + window_w)]
    #         average = np.sum(array) / (window_w * window_h)
    #         pixel = round(average / 255) * 255

    return np.array([
        np.array([
            round(np.sum(
                frame[round(y * window_h):round(y * window_h + window_h),
                      round(x * window_w):round(x * window_w + window_w)]
            ) / (window_w * window_h * 255)) * 255
            for x in range(width)
        ]) for y in range(height)
    ])                      

def reader(video: cv2.VideoCapture, width: int = 160, height: int = 90, start_frame: int = 1):
    for _ in range(start_frame):
        ret, frame = video.read()

    while ret:
        yield truncate_frame(frame, width, height)
        ret, frame = video.read()

    raise StopIteration


def create_frames_folder():
    path = Path("frames")
    full_path = path.resolve()
    new_directory = Path.cwd() / "frames"

    if not full_path.is_dir():
        new_directory.mkdir()

    return new_directory


def main():
    filename = "bad_apple.mp4"
    dimensions = 160, 90
    video = cv2.VideoCapture(f"{os.getcwd()}\{filename}")
    frame_reader = reader(video, *dimensions)
    path = create_frames_folder()

    for i, frame in enumerate(frame_reader):
        for _ in range(30):
            next(frame_reader)

        cv2.imwrite(str(path / f"frame{i * 30}.jpg"), frame)
        
        if i > 20:
            break
    

if __name__ == "__main__":
    main()
