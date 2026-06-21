#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/wait.h>
#include "hardware.h"

int execute_tpm_write(int target_version) {
    char version_str[12];
    snprintf(version_str, sizeof(version_str), "%d", target_version);

    pid_t pid = fork();
    if (pid == -1) {
        return -1; 
    }

    if (pid == 0) {
        char *args[] = {"/usr/sbin/tpmc", "write", "0x1008", version_str, NULL};
        execvp(args, args);
        exit(127);
    } else {
        int status;
        if (waitpid(pid, &status, 0) == -1) {
            return -2;
        }
        
        if (WIFEXITED(status)) {
            return WEXITSTATUS(status);
        }
        return -3;
    }
}
