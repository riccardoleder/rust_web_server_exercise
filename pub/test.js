
loadPage()

async function loadPage(){
  await loadData()
  
  document.getElementById("submit-btn").onclick = async (handler, event) => {
    await writeData()
  }

  // document.getElementById("text-input").onblur = async (handler, event) => {
  //   await writeRec("text")
  // }
}

async function loadData (){
  const response = await fetch("/data")
  const json = await response.json()
  document.getElementById("text-area").value = JSON.stringify(json)

  // const response2 = await fetch("/data/text")
  // const json2 = await response2.json()
  // document.getElementById("text-input").value = JSON.stringify(json2)
}

async function writeData() {
  const response = await fetch("/data", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: document.getElementById("text-area").value,
  });
  const json = await response.json()
  document.getElementById("text-area").value = JSON.stringify(json)
  alert("Data written!")
} 

// async function writeRec(rec) {
//   const response = await fetch(`/data/${rec}`, {
//     method: "POST",
//     headers: {
//       "Content-Type": "application/json",
//     },
//     body: document.getElementById("text-input").value,
//   });
//   const json = await response.json()
//   document.getElementById("text-input").value = JSON.stringify(json)
//   alert("text written!")
// } 