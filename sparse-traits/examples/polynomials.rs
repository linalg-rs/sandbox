pub use sparse_traits::*;
use std::fmt::Debug;

struct PolynomialSpace;
impl LinearSpace for PolynomialSpace {
    type F = f64;
    type E = Polynomial;
}

#[derive(Debug)]
struct Polynomial {
    monomial_coeffs: Vec<f64>,
}

impl Polynomial {
    fn from_monomial(monomial_coeffs: &[f64]) -> Self {
        Polynomial {
            monomial_coeffs: monomial_coeffs.to_owned(),
        }
    }
    fn eval(&self, x: f64) -> f64 {
        self.monomial_coeffs.iter().rev().fold(0., |r, c| r * x + c)
    }
}

#[derive(Debug)]
struct PolynomialView<'a> {
    monomial_coeffs: &'a [f64],
}
struct PolynomialViewMut<'a> {
    monomial_coeffs: &'a mut [f64],
}

impl Element for Polynomial {
    type Space = PolynomialSpace;
    type View<'a> = PolynomialView<'a> where Self: 'a ;
    type ViewMut<'a> = PolynomialViewMut<'a> where Self: 'a;

    fn view<'a>(&'a self) -> Self::View<'a> {
        PolynomialView {
            monomial_coeffs: &self.monomial_coeffs,
        }
    }
    fn view_mut<'a>(&'a mut self) -> PolynomialViewMut<'a> {
        PolynomialViewMut {
            monomial_coeffs: &mut self.monomial_coeffs,
        }
    }
}

struct PointwiseEvaluatorSpace;
impl LinearSpace for PointwiseEvaluatorSpace {
    type F = f64;
    type E = PointwiseEvaluate;
}
impl DualSpace for PointwiseEvaluatorSpace {
    type Space = PolynomialSpace;

    fn dual_pairing(&self, x: &Self::E, p: &<Self::Space as LinearSpace>::E) -> Result<Self::F> {
        Ok(x.scale * p.eval(x.x))
    }
}

struct PointwiseEvaluate {
    x: f64,
    scale: f64,
}
impl PointwiseEvaluate {
    fn new(x: f64) -> Self {
        PointwiseEvaluate { x, scale: 1. }
    }
}

impl Element for PointwiseEvaluate {
    type Space = PointwiseEvaluatorSpace;
    type View<'a> = &'a PointwiseEvaluate where Self: 'a;
    type ViewMut<'a> = &'a mut PointwiseEvaluate where Self: 'a;
    fn view<'a>(&'a self) -> Self::View<'a> {
        &self
    }
    fn view_mut<'a>(&'a mut self) -> Self::ViewMut<'a> {
        self
    }
}

#[derive(Debug)]
struct Derivative;
impl OperatorBase for Derivative {
    type Domain = PolynomialSpace;
    type Range = PolynomialSpace;
    fn as_apply(&self) -> Option<&dyn AsApply<Domain = Self::Domain, Range = Self::Range>> {
        Some(self)
    }
}
impl AsApply for Derivative {
    fn apply(&self, p: PolynomialView, dp: PolynomialViewMut) -> Result<()> {
        for (i, c) in p.monomial_coeffs[1..].iter().enumerate() {
            dp.monomial_coeffs[i] = (1. + i as f64) * c;
        }
        dp.monomial_coeffs[p.monomial_coeffs.len() - 1] = 0.;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Derivative, PointwiseEvaluate, PointwiseEvaluatorSpace, Polynomial, PolynomialSpace,
    };
    use sparse_traits::{AsApply, DualSpace, Element, OperatorBase, Result};

    #[test]
    fn test_poly_eval() {
        let p = Polynomial::from_monomial(&[1., 2., 3.]);
        assert_eq!(p.eval(2.), 17.);
    }

    #[test]
    fn test_dual() -> Result<()> {
        let ds = PointwiseEvaluatorSpace;
        let p = Polynomial::from_monomial(&[1., 2., 3.]);
        let n = PointwiseEvaluate::new(2.);
        let r = ds.dual_pairing(&n, &p)?;
        assert_eq!(r, 17.);
        Ok(())
    }

    #[test]
    fn test_derivative() -> Result<()> {
        let p = Polynomial::from_monomial(&[1., 2., 3.]);
        let mut dp = Polynomial::from_monomial(&[1., 1., 1.]);
        let d_ = Derivative;
        let d = &d_ as &dyn OperatorBase<Domain = PolynomialSpace, Range = PolynomialSpace>;
        d.apply(p.view(), dp.view_mut())?;
        assert_eq!(dp.monomial_coeffs, vec![2., 6., 0.]);
        Ok(())
    }
}
