import cv2

def getContours(img, original_img):
    contours, hierarchy = cv2.findContours(img, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_NONE)
    for cnt in contours:
        area = cv2.contourArea(cnt)
        if area > 100:
            cv2.drawContours(original_img, cnt, -1, (0, 255, 0), 2)

img = cv2.imread('sudoku.jpg')
imgGray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
imgBlur = cv2.GaussianBlur(imgGray, (5, 5), 3)
imgCanny = cv2.Canny(imgBlur, 50, 50)
imgCopy = img.copy()

getContours(imgCanny, imgCopy)

cv2.imwrite('extract.jpg', imgCopy)