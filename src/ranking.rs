//! ranking.rs
//! Relative Strength ranking for RitsTrend v0.1

use crate::analysis::StockAnalysis;

/// Computes relative strength ranks and percentile scores.
pub struct RelativeStrengthRanker;

impl RelativeStrengthRanker {
    /// Rank analyses by 6-month return (descending).
    pub fn rank(analyses: &mut [StockAnalysis]) {
        let mut order: Vec<usize> = analyses.iter().enumerate()
            .filter_map(|(i,a)| a.return6m.map(|r|(i,r)))
            .collect::<Vec<_>>();
        order.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let total = order.len();
        if total == 0 {
            return;
        }

        for (rank, (idx, _)) in order.into_iter().enumerate() {
            let rs_rank = rank + 1;
            let percentile =
                100.0 * ((total - rank) as f64) / (total as f64);

            analyses[idx].relative_strength_rank = Some(rs_rank);
            analyses[idx].relative_strength_score = Some(percentile);
        }
    }

    /// Returns cutoff rank for the top percentage.
    pub fn cutoff_rank(total: usize, top_percent: usize) -> usize {
        ((total * top_percent) + 99) / 100
    }

    /// True if the stock belongs to the top configured percentage.
    pub fn is_top_ranked(
        analysis: &StockAnalysis,
        total: usize,
        top_percent: usize,
    ) -> bool {
        match analysis.relative_strength_rank {
            Some(rank) => rank <= Self::cutoff_rank(total, top_percent),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::StockAnalysisBuilder;

    #[test]
    fn ranking_orders_returns_descending() {
        let mut items = vec![
            StockAnalysisBuilder::new().symbol("A").return6m(10.0).build(),
            StockAnalysisBuilder::new().symbol("B").return6m(40.0).build(),
            StockAnalysisBuilder::new().symbol("C").return6m(20.0).build(),
        ];

        RelativeStrengthRanker::rank(&mut items);

        assert_eq!(items[1].relative_strength_rank, Some(1));
        assert_eq!(items[2].relative_strength_rank, Some(2));
        assert_eq!(items[0].relative_strength_rank, Some(3));
    }
}
