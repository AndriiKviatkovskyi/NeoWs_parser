use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
struct NeoWsParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_string_valid() {
        let input = "\"2023-12-25\"";

        let result = NeoWsParser::parse(Rule::date_string, input);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "2023-12-25");
    }

    #[test]
    fn test_date_string_invalid() {
        let input = "\"12-25-2023\"";

        let result = NeoWsParser::parse(Rule::date_string, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_name_valid() {
        let input = "\"name\":\"Asteroid\"";

        let result = NeoWsParser::parse(Rule::name, input);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "Asteroid");
    }

    #[test]
    fn test_name_invalid() {
        let input = "name:\"Asteroid\"";

        let result = NeoWsParser::parse(Rule::name, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_kilometers_per_second_valid() {
        let input = "\"kilometers_per_second\":\"15.5\"";
        
        let result = NeoWsParser::parse(Rule::kilometers_per_second, input);
        
        assert!(result.is_ok());

        let parsed_value = result.unwrap().as_str();
        assert_eq!(parsed_value, "15.5");
    }

    #[test]
    fn test_kilometers_per_second_invalid() {
        let input = "\"kilometers_per_second\":\"1f5.5\"";
        
        let result = NeoWsParser::parse(Rule::kilometers_per_second, input);
        
        assert!(result.is_err());
    }


}
