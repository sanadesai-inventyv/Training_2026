let n=5;
for(let i=1;i<=n;i++){
    let row="";
    for(let j=1;j<=n;j++)
        row+=(i<j? (i<=n-j+1 ? i:n-j+1):(j<n-i+1?j:n-i+1))+"";
    console.log(row);
}