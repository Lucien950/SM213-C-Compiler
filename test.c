int  a;
int* p;
int  b[5];

int main() {
  a = 3;
  p = &a;
  *p = *p - 1;

  p = &b[0];
  p++;
  p[a] = b[a];
  *(p+3) = b[0];
}