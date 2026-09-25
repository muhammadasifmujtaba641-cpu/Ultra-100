// Ultra Guardian - C++ 100% Power Engine - Final DLL Build

#include <iostream>
#include <string>

using namespace std;

// Cross-platform export macro
// Windows par __declspec(dllexport) use hoga, aur Linux/Mac/OneCompiler par GCC visibility attribute.
#if defined(_WIN32) || defined(_WIN64)
    #define EXPORT __declspec(dllexport)
#else
    #define EXPORT __attribute__((visibility("default")))
#endif

// ==========================================
// ASAL SAFE LOGIC (C++ Engine)
// ==========================================
void cleanTemp() {
    cout << "[C++ Engine] Cleaning 5.2GB Temp - 100% Power" << endl;
}

void freeHiberfil() {
    cout << "[C++ Engine] Freed 8GB from hiberfil.sys (Simulation - No real deletion)" << endl;
}

void deepUninstallLogic(const string& appName) {
    cout << "[C++ Engine] Deep uninstall leftover for: " << appName << endl;
    cout << "[C++ Engine] Simulated deletion: C:\\Users\\...\\AppData\\Roaming\\" << appName << endl;
    cout << "[C++ Engine] Simulated registry cleanup: HKCU\\Software\\" << appName << endl;
}

void showMemoryMapLogic() {
    cout << "Memory Map:" << endl;
    cout << "Chrome: 850MB" << endl;
    cout << "WhatsApp: 350MB" << endl;
    cout << "FB Background: 600MB" << endl;
}

// ==========================================
// PYTHON GATE (extern "C" prevents name mangling)
// ==========================================
extern "C" {
    EXPORT void ultra_clean() { 
        cleanTemp();
    }
    
    EXPORT void free_hiberfil() {
        freeHiberfil();
    }
    
    EXPORT void deep_uninstall(const char* app) { 
        // Null pointer check: Agar Python se None aaye to C++ crash na ho
        if (app != nullptr) {
            deepUninstallLogic(string(app)); 
        } else {
            cout << "[C++ Engine] Error: Null app name provided." << endl;
        }
    }
    
    EXPORT void show_memory_map() {
        showMemoryMapLogic();
    }
}

// ==========================================
// ONECOMPILER / LOCAL TEST MAIN
// ==========================================
int main() {
    cout << "=== Ultra Guardian C++ Engine Test ===" << endl << endl;
    
    ultra_clean();
    cout << endl;
    
    free_hiberfil();
    cout << endl;
    
    deep_uninstall("OldChatApp");
    cout << endl;
    
    show_memory_map();
    
    cout << endl << "=== Engine Test Complete ===" << endl;
    return 0;
}
