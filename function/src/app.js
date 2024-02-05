exports.lambdaHandler = async (event) => {
    await new Promise(r => setTimeout(r, 60000));
};
