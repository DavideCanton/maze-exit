from pathlib import Path
import subprocess

folder = Path("img")

for n in folder.glob("*.png"):
    subprocess.run(
        [
            "target/debug/maze_exit_img_to_bin",
            n.resolve(),
            n.with_name(n.stem + ".bin").resolve(),
        ]
    )
