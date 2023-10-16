#include<bits/stdc++.h>
#include<pthread.h>

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
void* thread(void* ptr) {
    int x = *((int*)(ptr));

    if(check(x)) {
        printf("%d ", x);
    }

    pthread_exit(NULL);
    return NULL;
}

void solve_pthread(int n) {
    int* buf = (int*) malloc(sizeof(int) * (n + 1));
    pthread_t* id = (pthread_t*) malloc(sizeof(pthread_t) * (n + 1));

    // 1 is not prime
    for(int i = 2; i <= n; i++) {
        buf[i] = i;
        int ret = pthread_create(&(id[i]), NULL, thread, (void*)(&buf[i]));
        assert(ret == 0);
    }
    void* tmp;
    for(int i = 2; i <= n; i++) {
        pthread_join(id[i], &tmp);
    }

    free(buf);
    free(id);
}

/* main */
int main() {

    int n;
    scanf("%d", &n);

    solve_pthread(n);

    return 0;
}