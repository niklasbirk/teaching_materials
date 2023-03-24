#include "matrix.h"
#include <stdio.h>

matrix *new_matrix(m_type type)
{
    matrix *m = malloc(sizeof(matrix));

    m->type = type;
    m->values = malloc(sizeof(float *) * type.n);

    for (int i = 0; i < m->type.n; i++)
    {
        (m->values)[i] = malloc(sizeof(float) * type.m);
    }

    for (int i = 0; i < m->type.n; i++)
    {
        for (int j = 0; j < m->type.m; j++)
        {
            m->values[i][j] = 0;
        }
    }

    return m;
}

matrix *new_matrix_with_values(m_type type, float v[type.n][type.m])
{
    matrix *m = new_matrix(type);

    for (int i = 0; i < m->type.n; i++)
    {
        for (int j = 0; j < m->type.m; j++)
        {
            m->values[i][j] = v[i][j];
        }
    }

    return m;
}

void free_matrix(matrix *m)
{
    for (int i = 0; i < m->type.n; i++)
    {
        free((m->values)[i]);
    }

    free(m->values);
    free(m);
}

matrix *add(matrix *m1, matrix *m2)
{
    matrix *m = new_matrix(m1->type);

    for (int i = 0; i < m->type.n; i++)
    {
        for (int j = 0; j < m->type.m; j++)
        {
            m->values[i][j] = m1->values[i][j] + m2->values[i][j];
        }
    }

    return m;
}

matrix *multiply_with_scalar(float c, matrix *m)
{
    for (int i = 0; i < m->type.n; i++)
    {
        for (int j = 0; j < m->type.m; j++)
        {
            m->values[i][j] = c * m->values[i][j] ;
        }
    }

    return m;
}

matrix *multiply(matrix *m1, matrix *m2)
{
    if (m1->type.m != m2->type.n) return NULL;

    m_type type = {m1->type.n, m2->type.m};
    matrix *m = new_matrix(type);

    for (int i = 0; i < type.n; i++)
    {
        for (int j = 0; j < type.m; j++)
        {
            for (int k = 0; k < m1->type.m; k++)
            {
                m->values[i][j] += m1->values[i][k] * m2->values[k][j];
            }
        }
    }

    return m;
}