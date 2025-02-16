from PIL import Image
import os

"""
Get the paths that we neeed for where everything should be

Generalized so anyone can use it. Originally I had it hardcoded to my project, but hey if you want to use it, you can!
"""
script_dir = os.path.dirname(os.path.abspath(__file__))
project_root = os.path.abspath(os.path.join(script_dir, '..', '..'))
src_image = os.path.join(project_root, 'src', 'assets', 'PawPass_Logo.png')
icon_dir = script_dir

os.makedirs(icon_dir, exist_ok=True)

img = Image.open(src_image)


sizes = {
    "32x32.png": (32, 32),
    "128x128.png": (128, 128),
    "128x128@2x.png": (256, 256)
}

for filename, size in sizes.items():
    resized = img.resize(size, Image.Resampling.LANCZOS)
    resized.save(os.path.join(icon_dir, filename))
    print(f"Created {filename}")

# Windows ICO file
ico_sizes = [(32, 32), (128, 128), (256, 256)]
ico_path = os.path.join(icon_dir, "icon.ico")
img.save(ico_path, format='ICO', sizes=ico_sizes)
print("Created icon.ico")

# Mac ICNS file
icns_path = os.path.join(icon_dir, "icon.icns")
img.save(icns_path, format='ICNS')
print("Created icon.icns")

print("Icon generation complete!")
