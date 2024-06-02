function PlayAll() {
  var xpath = "//span[text()='Play all']";
  document
    .evaluate(xpath, document, null, XPathResult.FIRST_ORDERED_NODE_TYPE, null)
    .singleNodeValue
    .parentNode
    .parentNode
    .click();
}

function PlayPause() {// #top-player-bar
  document.getElementById('play-pause-button').click();
}

function PlayPauseClick() {
  if(document.getElementById('layout').getAttributeNames().includes('player-visible')){
    PlayPause();
  }else{
    PlayAll();
  }
}

function WaitWindow() {
  window.alert("poczekaj");
}

function Checker(){
  let metadata = navigator.mediaSession.metadata;
  if(!metadata){ return }
  meta = JSON.stringify({
    artist: metadata.artist,
    title: metadata.title,
    album: metadata.album,
    state: navigator.mediaSession.playbackState
  })
  if(meta !== navigator.oldmeta) {
    navigator.oldmeta = meta;
    window.ipc.postMessage(meta);
  }
}

setInterval(Checker, 250);

// function FixBottomPlayer(event) {
//   if((event.target.innerWidth || window.innerWidth) < 640) {
//     console.log(event.target.innerWidth || window.innerWidth);
//     document.querySelector('ytmusic-player-bar.ytmusic-app').removeAttribute('is-mweb-player-bar-modernization-enabled');
//     document.querySelector('ytmusic-player-bar.ytmusic-app>#left-controls').style.display = "none";
//     document.querySelector('ytmusic-player-bar.ytmusic-app>#right-controls').style.display = "none";
//     document.querySelector('ytmusic-player-bar.ytmusic-app>#right-controls-mweb').style.display = "flex";
//     document.querySelector('ytmusic-player-bar.ytmusic-app button[aria-label="Action menu"]').style.display = "none";
//   }else{
//     document.querySelector('ytmusic-player-bar.ytmusic-app>#left-controls').style.display = "";
//     document.querySelector('ytmusic-player-bar.ytmusic-app>#right-controls').style.display = "";
//     document.querySelector('ytmusic-player-bar.ytmusic-app button').style.display = "";
//     document.querySelector('ytmusic-player-bar.ytmusic-app>#right-controls-mweb').style.display = "none";
//     document.querySelector('ytmusic-player-bar.ytmusic-app button[aria-label="Action menu"]').style.display = "";
//   }
// }
//
// //window.addEventListener("load", FixBottomPlayer);
// window.addEventListener("resize", FixBottomPlayer);
//
