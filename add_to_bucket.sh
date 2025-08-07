cd bucket
i=1000
for f in *.yaml; do
  if [[ $f != bucket* ]]; then
    printf -v newf 'bucket (%d).yaml' "$i"
    mv -- "$f" "$newf"
    i=$((i + 1))
  fi  
done