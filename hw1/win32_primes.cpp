#include<bits/stdc++.h>
#include<windows.h>

/* Common */
bool check(int x) {
    for(int i = 2, lim = sqrt(x); i <= lim; i++) {
        if(x % i == 0) {
            return false;
        }
    }
    return true;
}

/* pthread */
DWORD WINAPI thread(LPVOID lpParam) {
    int x = *((int*)(lpParam));

    if(check(x)) {
        printf("%d ", x);
    }

    return 0;
}

void solve_win32(int n) {
    int* buf = (int*) malloc(sizeof(int) * (n + 1));
    HANDLE* id = (HANDLE*) malloc(sizeof(HANDLE) * (n + 1));

    // 1 is not prime
    for(int i = 2; i <= n; i++) {
        buf[i] = i;
        id[i] = CreateThread(NULL, 0, thread, (LPVOID)(&buf[i]), 0, NULL);
    }

    for(int i = 2; i <= n; i++) {
        WaitForSingleObject(id[i], INFINITE);
        CloseHandle(id[i]);
    }

    free(buf);
    free(id);
}

/* main */
int main() {

    int n;
    scanf("%d", &n);

    solve_win32(n);

    return 0;
}