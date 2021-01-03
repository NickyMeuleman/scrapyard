# code from @RylandeFM, confirmed working, going to base my part2 method on this solution

from functools import reduce
from math import sqrt
import re

inputString = open("./input.txt", "r").read().splitlines()

def parseAllPictures():
    currentID, picture, pictures = "", [], {}
    for line in inputString:
        if "Tile" in line:
            currentID = line.split(" ")[1].split(":")[0]
        elif line != "":
            picture.append(line)
        else:
            pictures[currentID] = picture
            picture = []
    pictures[currentID] = picture
    return pictures

def getAllOtherEdges(pictures, picID):
    edges = set()
    for picture in {x: y for x, y in pictures.items() if x != picID}.values():
        edges.update(getPotentialEdges(picture))
    return edges

def getPotentialEdges(picture):
    edges = [(picture[0], picture[0][::-1])]
    right = "".join([line[-1] for line in picture])
    edges.append((right, right[::-1]))
    edges.append((picture[-1], picture[-1][::-1]))
    left = "".join([line[0] for line in picture])
    edges.append((left, left[::-1]))
    return edges

def countCommonEdges():
    pictures, numberCommonEdges = parseAllPictures(), {}
    for picId, picture in pictures.items():
        allEdges, edgeList = getAllOtherEdges(pictures, picId), getPotentialEdges(picture)
        numberCommonEdges[picId] = sum([edge in allEdges or edge[::-1] in allEdges for edge in edgeList])
    print(reduce(lambda x, y: x * y, [int(x) for x, y in numberCommonEdges.items() if y == 2]))
    return pictures, numberCommonEdges

def getPossibleOrientations(picture):
    possibles = [picture]  # normal
    possibles.append(picture[::-1])  # flipped horizontal axis
    possibles.append([line[::-1] for line in picture])  # flipped vertical axis
    possibles.append([line[::-1] for line in picture][::-1])  # flipped both axes
    possibles.append(["".join(list(elem)) for elem in zip(*picture[::-1])])  # normal rotated 90 deg
    possibles.append(["".join(list(elem)) for elem in zip(*picture)])  # flipped horizontal rotated 90 deg
    possibles.append(["".join(list(elem)) for elem in zip(*[line[::-1] for line in picture][::-1])])  # flipped vertical rotated 90 deg
    possibles.append(["".join(list(elem)) for elem in zip(*[line[::-1] for line in picture])])  # flipped both rotated 90 deg
    return possibles

def findPictureToRight(pictures, picture, pieces):
    right = "".join([line[-1] for line in picture])
    for pieceID in pieces:
        for possible in getPossibleOrientations(pictures[pieceID]):
            left = "".join([line[0] for line in possible])
            if left == right: return pieceID, possible

def findPictureToBottom(pictures, picture, pieces):
    bottom = picture[-1]
    for pieceID in pieces:
        for possible in getPossibleOrientations(pictures[pieceID]):
            top = possible[0]
            if bottom == top: return pieceID, possible

def buildImage(pictures, numberCommonEdges, startCorner):
    dimension = int(sqrt(len(pictures)))
    pictureMap = [[""] * dimension for _ in range(dimension)]
    cornerPieces = [x for x, y in numberCommonEdges.items() if y == 2]
    edgePieces = [x for x, y in numberCommonEdges.items() if y == 3]
    centerPieces = [x for x, y in numberCommonEdges.items() if y == 4]
    pictureMap[0][0] = cornerPieces[startCorner]
    orientedPictures = {}
    # get top left corner
    allEdges = getAllOtherEdges(pictures, cornerPieces[startCorner])
    for picture in getPossibleOrientations(pictures[cornerPieces[startCorner]]):
        if [edge in allEdges or edge[::-1] in allEdges for edge in getPotentialEdges(picture)] == [False, True, True, False]:
            orientedPictures[cornerPieces[startCorner]] = picture
            break
    # fill top row
    nextPicture = orientedPictures[cornerPieces[startCorner]]
    cornerPieces.remove(cornerPieces[startCorner])
    for c in range(1, dimension - 1):
        pieceID, nextPicture = findPictureToRight(pictures, nextPicture, edgePieces)
        pictureMap[0][c] = pieceID
        orientedPictures[pieceID] = nextPicture
        edgePieces.remove(pieceID)
    pieceID, nextPicture = findPictureToRight(pictures, nextPicture, cornerPieces)
    pictureMap[0][dimension - 1] = pieceID
    orientedPictures[pieceID] = nextPicture
    cornerPieces.remove(pieceID)
    # fill middle rows
    for r in range(1, dimension - 1):
        nextPicture = orientedPictures[pictureMap[r - 1][0]]
        pieceID, nextPicture = findPictureToBottom(pictures, nextPicture, edgePieces)
        pictureMap[r][0] = pieceID
        orientedPictures[pieceID] = nextPicture
        edgePieces.remove(pieceID)
        for c in range(1, dimension - 1):
            pieceID, nextPicture = findPictureToRight(pictures, nextPicture, centerPieces)
            pictureMap[r][c] = pieceID
            orientedPictures[pieceID] = nextPicture
            centerPieces.remove(pieceID)
        pieceID, nextPicture = findPictureToRight(pictures, nextPicture, edgePieces)
        pictureMap[r][dimension - 1] = pieceID
        orientedPictures[pieceID] = nextPicture
        edgePieces.remove(pieceID)
    # fill bottom row
    r = dimension - 1
    nextPicture = orientedPictures[pictureMap[r - 1][0]]
    pieceID, nextPicture = findPictureToBottom(pictures, nextPicture, cornerPieces)
    pictureMap[r][0] = pieceID
    orientedPictures[pieceID] = nextPicture
    cornerPieces.remove(pieceID)
    for c in range(1, dimension - 1):
        pieceID, nextPicture = findPictureToRight(pictures, nextPicture, edgePieces)
        pictureMap[r][c] = pieceID
        orientedPictures[pieceID] = nextPicture
        edgePieces.remove(pieceID)
    pieceID, nextPicture = findPictureToRight(pictures, nextPicture, cornerPieces)
    pictureMap[r][dimension - 1] = pieceID
    orientedPictures[pieceID] = nextPicture
    cornerPieces.remove(pieceID)
    return pictureMap, orientedPictures

def removeBorder(picture):
    result, length = [], len(picture[0])
    for r in range(1, length-1):
        result.append(picture[r][1:length-1])
    return result

def removeGaps(orientedPictures, pictureMap):
    image, length = [], len(orientedPictures[pictureMap[0][0]])
    for i, row in enumerate(pictureMap):
        for idx in range(length):
            line = ""
            for picID in row:
                line += orientedPictures[picID][idx]
            image.append(line)
    return image

def findCorrectRotation(finalImage, body, head, tail):
    for image in getPossibleOrientations(finalImage):
        for idx, line in enumerate(image):
            bodySearch = body.search(line)
            if bodySearch:
                start, end = bodySearch.start(), bodySearch.end()
                if tail.search(image[idx + 1][start:end + 1]) and head.search(image[idx - 1][start:end + 1]):
                    return image

def lineHasSnake(idx, line, finalImage, body, head, tail):
    if body.search(line):
        bodySearch = body.search(line)
        if bodySearch:
            start, end = bodySearch.start(), bodySearch.end()
            return tail.search(finalImage[idx + 1][start:end + 1]) and head.search(finalImage[idx - 1][start:end + 1])
    return False

def getWaterRoughness():
    pictures, numberCommonEdges = countCommonEdges()
    pictureMap, orientedPictures = buildImage(pictures, numberCommonEdges, 0)
    for picId, picture in orientedPictures.items():
        orientedPictures[picId] = removeBorder(picture)
    body = re.compile("#[#.]{4}##[#.]{4}##[#.]{4}###")
    head = re.compile("[#.]{18}#[#.]")
    tail = re.compile("[#.]#[#.]{2}#[#.]{2}#[#.]{2}#[#.]{2}#[#.]{2}#[#.]{3}")
    finalImage = findCorrectRotation(removeGaps(orientedPictures, pictureMap), body, head, tail)
    headIdxs, bodyIdxs, tailIdxs = [18], [0, 5, 6, 11, 12, 17, 18, 19], [1, 4, 7, 10, 13, 16]
    for idx, line in enumerate(finalImage):
        while lineHasSnake(idx, line, finalImage, body, head, tail):
            bodySearch = body.search(line)
            start, end = bodySearch.start(), bodySearch.end()
            # replace head
            repLine = list(finalImage[idx - 1])
            repLine[headIdxs[0] + start] = "O"
            finalImage[idx - 1] = "".join(repLine)
            # replace body
            repLine = list(finalImage[idx])
            for bodyIdx in bodyIdxs:
                repLine[bodyIdx + start] = "O"
            finalImage[idx] = "".join(repLine)
            # replace tail
            repLine = list(finalImage[idx + 1])
            for tailIdx in tailIdxs:
                repLine[tailIdx + start] = "O"
            finalImage[idx + 1] = "".join(repLine)
            # update line, there might be more than one snake
            line = finalImage[idx]
    print(sum([line.count("#") for line in finalImage]))

getWaterRoughness()