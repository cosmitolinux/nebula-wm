#include <iostream> // source code not made by cosmito heres a note by cosmito "so sory guys im lazy azf"
#include <cstdlib>
#include <sys/stat.h>
#include <string>

// gotta make sure the folder exists or itll 100% crash
void ensure_dir(const char* path) {
    struct stat info;
    if (stat(path, &info) != 0) {
        std::string cmd = "mkdir -p ";
        cmd += path;
        system(cmd.c_str());
    }
}

int main() {
    // n3bu1a shell loading up... lets get it
    std::cout << "--- n3bu1a shell v1.0 ---\n";

    // setting up the screenshot path so it dont get messy
    const char* home = getenv("HOME");
    std::string shot_path = std::string(home) + "/Screenshots";
    ensure_dir(shot_path.c_str());

    // pkill the old stuff so we dont have ghost bars
    system("pkill waybar");
    system("pkill awww/swww"); // depends on your wallpaper engine

    // actually launching the ui now
    std::cout << "[n3bu1a] firing up waybar...\n";
    system("waybar &");
    
    std::cout << "[n3bu1a] we vibin. shell is ready.\n";
    return 0;
}
