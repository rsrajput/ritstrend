use crate::analysis::StockAnalysis;

/// Relative Strength ranking for screened stocks.
#[derive(Debug, Default)]
pub struct RankingEngine;

impl RankingEngine {
    /// Create a new ranking engine instance.
    pub fn new() -> Self {
        Self
    }

    /// Rank the supplied analyses by six-month return descending.
    pub fn rank(analyses: &mut [StockAnalysis]) {
        analyses.sort_by(|left, right| {
            right
                .return6m
                .unwrap_or_default()
                .partial_cmp(&left.return6m.unwrap_or_default())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for (index, analysis) in analyses.iter_mut().enumerate() {
            analysis.relative_strength_rank = Some(index + 1);
            analysis.relative_strength_score = analysis.return6m;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::StockAnalysisBuilder;

    #[test]
    fn ranks_analyses_by_return6m_descending() {
        let mut analyses = vec![
            StockAnalysisBuilder::new().symbol("LOW").return6m(0.05).build(),
            StockAnalysisBuilder::new().symbol("HIGH").return6m(0.20).build(),
            StockAnalysisBuilder::new().symbol("MID").return6m(0.10).build(),
        ];

        RankingEngine::rank(&mut analyses);

        assert_eq!(analyses[0].symbol, "HIGH");
        assert_eq!(analyses[0].relative_strength_rank, Some(1));
        assert_eq!(analyses[1].symbol, "MID");
        assert_eq!(analyses[1].relative_strength_rank, Some(2));
        assert_eq!(analyses[2].symbol, "LOW");
        assert_eq!(analyses[2].relative_strength_rank, Some(3));
    }
}
