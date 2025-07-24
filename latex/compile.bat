@echo off
echo Compiling LaTeX document...
pdflatex meas_filter.tex
echo First pass completed.
pdflatex meas_filter.tex
echo Second pass completed. Cross-references resolved.
echo.
echo Compilation finished! PDF generated: meas_filter.pdf
echo.
pause
