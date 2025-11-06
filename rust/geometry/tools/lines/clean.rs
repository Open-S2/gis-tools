use crate::geometry::ClampWGS84Point;
use alloc::{vec, vec::Vec};
use libm::fabs;
use s2json::{GetXY, SetXY};

/// Removes superfluous/collinear points from a collection of linestrings
///
/// ## Parameters
/// - `lines`: the linestring to clean
/// - `eps`: the tolerance. Defaults to `1e-12`
/// - `clean_wgs84`: if true, clean WGS84 points to be valid WGS84 points
///
/// ## Returns
/// The cleaned linestrings
pub fn clean_linestrings<P: GetXY + SetXY + Clone + PartialEq>(
    lines: &Vec<Vec<P>>,
    is_poly: bool,
    eps: Option<f64>,
    clean_wgs84: bool,
) -> Option<Vec<Vec<P>>> {
    let res: Vec<Vec<P>> =
        lines.into_iter().filter_map(|p| clean_linestring(p, is_poly, eps, clean_wgs84)).collect();
    if res.is_empty() { None } else { Some(res) }
}

/// Removes superfluous/collinear points from a linestring
///
/// ## Parameters
/// - `line`: the linestring to clean
/// - `eps`: the toleranc to check if the segments are superfluous/collinear. Defaults to `1e-12`
/// - `clean_wgs84`: if true, clean WGS84 points to be valid WGS84 points
///
/// ## Returns
/// The cleaned linestring
pub fn clean_linestring<P: GetXY + SetXY + Clone + PartialEq>(
    line: &Vec<P>,
    is_poly: bool,
    eps: Option<f64>,
    clean_wgs84: bool,
) -> Option<Vec<P>> {
    if if is_poly { line.len() < 4 } else { line.len() < 2 } {
        return None;
    }
    let eps = eps.unwrap_or(1e-12);
    // First remove all duplicates
    let mut no_dups: Vec<&P> = vec![&line[0]];
    for i in 1..line.len() {
        if line[i] != *no_dups[no_dups.len() - 1] {
            no_dups.push(&line[i]);
        }
    }
    // Then remove superfluous/collinear points
    let mut cleaned: Vec<P> = vec![no_dups[0].clone()];
    for i in 1..no_dups.len() - 1 {
        let prev = &no_dups[i - 1];
        let curr = &no_dups[i];
        let next = &no_dups[i + 1];
        let area = (curr.y() - prev.y()) * (next.x() - curr.x())
            - (curr.x() - prev.x()) * (next.y() - curr.y());
        if fabs(area) > eps {
            cleaned.push((*curr).clone());
        }
    }
    cleaned.push(no_dups[no_dups.len() - 1].clone());
    // check again if linestring is valid
    if if is_poly { cleaned.len() < 4 } else { cleaned.len() < 2 } {
        return None;
    }
    // if user want's valid WGS84 points let's fix them
    if clean_wgs84 {
        cleaned.iter_mut().for_each(|p| p.clamp_wgs84());
    }

    Some(cleaned)
}
