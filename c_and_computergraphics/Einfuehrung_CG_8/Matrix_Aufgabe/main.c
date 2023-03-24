#include <stdio.h>

#include "matrix/matrix.h"

void print_matrix(matrix *m);

void add_matrix();
void multiply_matrix_with_scalar();
void multiply_matrices();

int main()
{
    add_matrix();
    printf("\n");
    multiply_matrix_with_scalar();
    printf("\n");
    multiply_matrices();

    return 0;
}

void add_matrix()
{
    m_type type = {2, 2};
    float v[2][2] = {
            {1, 2},
            {3, 4}
    };
    matrix *m1 = new_matrix_with_values(type, v);
    matrix *m2 = new_matrix_with_values(type, v);

    matrix *m = add(m1, m2);

    print_matrix(m);

    free_matrix(m1);
    free_matrix(m2);
    free_matrix(m);
}

void multiply_matrix_with_scalar()
{
    m_type type = {2, 2};
    float v[2][2] = {
            {1, 2},
            {3, 4}
    };
    matrix *m = new_matrix_with_values(type, v);

    m = multiply_with_scalar(0.5f, m);

    print_matrix(m);

    free_matrix(m);
}

void multiply_matrices()
{
    m_type type = {2, 2};
    float v[2][2] = {
            {1, 2},
            {3, 4}
    };
    matrix *m1 = new_matrix_with_values(type, v);
    matrix *m2 = new_matrix_with_values(type, v);

    matrix *m = multiply(m1, m2);

    print_matrix(m);

    free_matrix(m1);
    free_matrix(m2);
    free_matrix(m);
}

void print_matrix(matrix *m)
{
    for (int i = 0; i < m->type.n; i++)
    {
        printf("[");
        for (int j = 0; j < m->type.m; j++)
        {
            printf(" %6.3f ", m->values[i][j]);
        }
        printf("]\n");
    }
}
