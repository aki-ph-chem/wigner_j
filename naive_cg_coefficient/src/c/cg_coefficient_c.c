#include <stdlib.h>
#include <stdio.h>
#include <math.h>

struct CGCoefficient{
    int j_1;
    int m_1;
    int j_2;
    int m_2;
    int j_3;
    int m_3;
};

void show_j_list(const struct CGCoefficient* cg) {
    printf("(j_1, m_1) = (%d, %d)\n", cg->j_1, cg->m_1);
    printf("(j_2, m_2) = (%d, %d)\n", cg->j_2, cg->m_2);
    printf("(j_3, m_3) = (%d, %d)\n", cg->j_3, cg->m_3);
}

double factorial(double n){
  if( n < 0 ) return -100.;
  return (n == 1. || n == 0.) ? 1. : factorial(n-1) * n ;
}

double min(double x_1, double x_2) {
    if(x_1 < x_2)  {
        return x_1;
    } else {
        return x_2;
    }
}

double CGcoeff(double J, double m, double J1, double m1, double J2, double m2){
  // (J1,m1) + (J2, m2) = (J, m)
 
  if( m != m1 + m2 ) return 0;
 
  double Jmin = fabs(J1 - J2);
  double Jmax = J1+J2;
 
  if( J < Jmin || Jmax < J ) return 0;
 
  double s0 = (2*J+1.) * factorial(J+J1-J2) * factorial(J-J1+J2) * factorial(J1+J2-J) / factorial(J+J1+J2 + 1.);
  s0 = sqrt(s0);
 
  double s = factorial(J +m ) * factorial(J -m);
  double s1 = factorial(J1+m1) * factorial(J1-m1);
  double s2 = factorial(J2+m2) * factorial(J2-m2);
  s = sqrt(s * s1 * s2);
 
  //printf(" s0, s = %f , %f \n", s0, s);
 
  int kMax = min( min( J1+J2-J, J1 - m1), J2 + m2);
 
  double CG = 0.;
  for( int k = 0; k <= kMax; k++){
    double k1 = factorial(J1+J2-J-k);
    double k2 = factorial(J1-m1-k);
    double k3 = factorial(J2+m2-k);
    double k4 = factorial(J - J2 + m1 +k);
    double k5 = factorial(J - J1 - m2 +k);
    double temp = pow(-1, k) / (factorial(k) * k1 * k2 * k3 * k4 * k5);
    if( k1 == -100. || k2 == -100. || k3 == -100. || k4 == -100. || k5 == -100. ) temp = 0.;
 
    //printf(" %d | %f \n", k, temp);
    CG += temp;
  }
 
  return s0 * s * CG;
 
}
