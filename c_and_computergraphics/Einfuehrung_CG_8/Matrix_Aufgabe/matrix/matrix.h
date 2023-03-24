#include <stdlib.h>

typedef struct m_type {
    int n, m;
} m_type;

typedef struct matrix {
    m_type type;
    float **values;
} matrix;

matrix *new_matrix(m_type type);
matrix *new_matrix_with_values(m_type type, float v[type.n][type.m]);

void free_matrix(matrix *m);

matrix *add(matrix *m1, matrix *m2);

matrix *multiply_with_scalar(float c, matrix *m);

matrix *multiply(matrix *m1, matrix *m2);