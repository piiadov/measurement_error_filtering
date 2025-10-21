# Measurement Error Filtering

**[Read the Paper (PDF)](latex/meas_filter.pdf)**

This repository contains research on filtering input data errors in model error calculations, specifically examining the impact of measurement uncertainty on Mean Absolute Error (MAE) estimation.

## Contents

- **`latex/`** - LaTeX source code for the research paper
  - `meas_filter.tex` - Main research paper
  - `arxiv.sty` - LaTeX style file
  - `img/` - Figures and illustrations
  - `compile.bat` - Windows batch script for easy PDF compilation

- **`simulator/`** - Simple rust-based simulation code
  - Implementation of uncertainty models and MAE calculations

## Research Abstract

This paper examines the impact of measurement uncertainty on Mean Absolute Error (MAE) calculations in statistical analysis and model validation. We derive analytical expressions for the distribution of absolute differences between observations under various uncertainty models, including delta function distribution for data without uncertainty and normal distributions for measurement errors.

## Citation

If you use this work in your research, do not hesitate to cite:

```
Piiadov, V. (2025). Filtering Input Data Errors in Model Error Calculations: 
Impact of Measurement Uncertainty on Mean Absolute Error Estimation. 
GitHub repository: https://github.com/piiadov/measurement_error_filtering
```

## License

This work is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

**Vasilii Piiadov**  
[LinkedIn](https://www.linkedin.com/in/vasilii-piiadov/)

## Contributing

This is a research repository. Feel free to open issues for questions or discussions.

