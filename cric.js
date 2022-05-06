async function fetchFromOffset(offset) {
  return await fetch(
    "https://api.cricapi.com/v1/countries?apikey=[your api key]&offset=" +
      offset
  )
    .then((data) => data.json())
    .then((data) => {
      if (data.status != "success") {
        alert("Failed");
        return;
      }
      let datarray = data.data;
      if (!datarray) return [];
      else if (offset >= data.info.totalRows) return datarray;
      else
        return fetchFromOffset(offset + 25).then(function (data) {
          return datarray.concat(data);
        });
    })
    .catch((e) => console.log);
}
fetchFromOffset(0)
  .then(function (data) {
    console.log("Complete data got!", data);
  })
  .catch((e) => console.log);
