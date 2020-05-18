BUCKET=experiment.bens-stuff.net

for f in ./dist/*.html
do
    filename="$(basename -- $f)"
    aws s3api put-object --bucket $BUCKET --key $filename --body $f --content-type 'text/html; charset=utf-8'
done

for f in ./dist/*.js
do
    filename="$(basename -- $f)"
    aws s3api put-object --bucket $BUCKET --key $filename --body $f --content-type 'text/javascript; charset=utf-8'
done
