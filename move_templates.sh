mv ~/Downloads/OldTemplates.zip ./
mv ~/Downloads/NewTemplates.zip ./
rm -r ./compare_old
rm -r ./compare_new
unzip ./OldTemplates.zip -d ./compare_old
unzip ./NewTemplates.zip -d ./compare_new
