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


//ytmusic-player-bar -> is-mweb-player-page-modernization-enabled
