let array=[5,15,10,20];
function arrayfunc(arr){
    arr.shift();
    return secondfunc(arr);
}
function secondfunc(arr1){
    let firstelement=arr1[0];
    let restarray=arr1.slice(1);
    let array2=[];
    array2.push(firstelement);
    for(let i=0;i<restarray.length;i++){
        array2.push(restarray[i]);
}
return new Promise(function(resolve,reject){
let sum=0;
for(let i=0;i<array2.length;i++){
    sum=sum+array2[i];
}
if(sum>35){
    resolve("Resolved:sum is"+sum);
}else{
    reject("rejected:sum is"+sum);
}
});
}
arrayfunc(array)
    .then(function(result){
        console.log(result);
    })
    .catch(function(error){
        console.log(error);
    })

