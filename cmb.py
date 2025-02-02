from io import TextIOWrapper
from os import listdir, remove
from os.path import isfile, isdir, join


def reduct(dir: str, file: TextIOWrapper):
    for sub_dir in listdir(dir):
        full_dir = join(dir, sub_dir)

        if isfile(full_dir) and full_dir.endswith(".gitignore"):
            file.write(
                f"\n# {full_dir.removeprefix("gitignore_src/").removesuffix(".gitignore")}\n"
            )
            for line in open(full_dir).readlines():
                line = line.strip().strip("\n")

                if line.startswith("#"):
                    file.write(f"Comment({line.removeprefix("#").strip()})\n")
                else:
                    if line:
                        file.write(f"Item({line})\n")

        elif isdir(full_dir):
            reduct(full_dir, file)


name = f"src/ignores.txt"
try:
    with open(name, "w") as f:
        reduct("gitignore_src", f)
except Exception as e:
    remove(name)
    print(e)
