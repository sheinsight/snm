



(async () => {
    console.log("start hello world! this log by nodejs");
    await new Promise((resolve, reject) => {
        setTimeout(() => {
            console.log("wait 2 seconds");
            resolve();
        }, 4000);
    }
    );
    console.log("end hello world! this log by nodejs");
})()