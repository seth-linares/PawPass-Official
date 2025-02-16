from moviepy import VideoFileClip
import os

def check_permissions(path):
    """
    Makes sure we can write to the destination folder.
    
    First, we'll check if the folder exists and create it if needed.
    Then we'll do a quick test write to make sure we have the right permissions.

    Depending on where you want to save the GIF it can be a pain. I initially wanted to save to desktop and that was not allowed,
    so I decided to just add the default path to the same folder as the script.
    """
    directory = os.path.dirname(path)
    if not os.path.exists(directory):
        try:
            os.makedirs(directory)
            return True
        except (OSError, PermissionError) as e:
            print(f"Error creating directory: {str(e)}")
            return False
    
    try:
        test_file = os.path.join(directory, 'test_write')
        with open(test_file, 'w') as f:
            f.write('test')
        os.remove(test_file)
        return True
    except (OSError, PermissionError):
        return False

def convert_video_to_gif(video_path, gif_path, resize_factor=1.0):
    """
    Turns your video into a high-quality GIF.
    
    We'll first check if we can write to your chosen location, then open the video
    and convert it. If you want to resize the video, we'll handle that too.
    The output GIF will run at 20 frames per second for smooth playback.
    """
    try:
        if not check_permissions(gif_path):
            raise PermissionError(f"No write permission for path: {gif_path}")

        with VideoFileClip(video_path) as video_clip:
            if resize_factor != 1.0:
                video_clip = video_clip.resize(resize_factor)
            
            output_dir = os.path.dirname(gif_path)
            if not os.path.exists(output_dir):
                os.makedirs(output_dir)
            
            video_clip.write_gif(gif_path, fps=20)
            
        return True
    except Exception as e:
        print(f"Error converting video: {str(e)}")
        return False

def get_default_output_path(video_path):
    """
    This is just figuring out where to save your GIF if you don't specify a location
    
    We'll take the name of your video file and create a GIF with the same name,
    saving it in the same folder as this script.
    """
    base_name = os.path.splitext(os.path.basename(video_path))[0]
    return os.path.join(os.path.dirname(__file__), f"{base_name}.gif")

def main():
    """
    Our main function will guide you through the process of converting a video to a GIF.

    This script is MUCH more preferable and higher-quality than using online converters like ezgif.
    It looks terrible for anything other than tiny files!!
    
    Here's what we'll do:
    1. Ask you for your video file
    2. Let you choose where to save the GIF
    3. Ask if you want to resize it
    4. Convert it and let you know when it's done
    """
    print("Welcome to the Video to GIF converter!")
    
    video_path = input("Enter the path to your video file: ").strip('"')
    
    default_path = get_default_output_path(video_path)
    gif_path = input(f"Enter the path for the output GIF (press Enter for default: {default_path}): ").strip('"')
    
    if not gif_path:
        gif_path = default_path
    
    resize = input("Enter resize factor (1.0 for original size, 0.5 for half size): ")
    
    if not os.path.exists(video_path):
        print("Error: Video file does not exist!")
        return
    
    try:
        resize_factor = float(resize)
    except ValueError:
        print("Error: Invalid resize factor! Using default 1.0")
        resize_factor = 1.0
    
    print("Converting video to GIF...")
    success = convert_video_to_gif(video_path, gif_path, resize_factor)
    
    if success:
        print(f"Conversion complete! GIF saved to: {gif_path}")
    else:
        print("Conversion failed! Please ensure you have write permissions and specified a valid path.")

if __name__ == "__main__":
    main()
