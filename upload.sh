if cargo v5 upload --upload-strategy=monolith; then
    echo "starting terminal"
    cargo v5 terminal
else
    if cargo v5 upload --upload-strategy=monolith; then
        echo "starting terminal"
        cargo v5 terminal
    fi
fi