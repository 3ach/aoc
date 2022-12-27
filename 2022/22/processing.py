from PIL import Image, ImageDraw

images = []
with open('./out', 'rt') as input:
    maps = "".join(input.readlines()).split("\n\n")
    for mnum, map in enumerate(maps):
        if mnum % 100 == 0:
            print(f"{mnum} of {len(maps)} maps processed")
        map = map.split("\n")
        width = max(len(row) for row in map)
        height = len(map)
        map_image = Image.new("RGB", (width * 2, height * 2))
        map_image_pxl = map_image.load()

        for ridx in range(height):
                row = map[ridx]
                for (idx, c) in enumerate(row):
                    if c == ".":
                        map_image_pxl[2 * idx, 2 * ridx] = (255, 255, 255)
                        map_image_pxl[2 * idx + 1, 2 * ridx] = (255, 255, 255)
                        map_image_pxl[2 * idx, 2 * ridx + 1] = (255, 255, 255)
                        map_image_pxl[2 * idx + 1, 2 * ridx + 1] = (255, 255, 255)

                    if c == "#":
                        map_image_pxl[2 * idx, 2 * ridx] = (0, 255, 0)
                        map_image_pxl[2 * idx + 1, 2 * ridx] = (0, 255, 0)
                        map_image_pxl[2 * idx, 2 * ridx + 1] = (0, 255, 0)
                        map_image_pxl[2 * idx + 1, 2 * ridx + 1] = (0, 255, 0)

                    if c == "X":
                        map_image_pxl[2 * idx, 2 * ridx] = (255, 0, 0)
                        map_image_pxl[2 * idx + 1, 2 * ridx] = (255, 0, 0)
                        map_image_pxl[2 * idx, 2 * ridx + 1] = (255, 0, 0)
                        map_image_pxl[2 * idx + 1, 2 * ridx + 1] = (255, 0, 0)
        
        images.append(map_image)

    images[0].save("./route.gif", save_all=True, append_images=images[1:], optimize=False, duration=2000, loop=0)



                    

