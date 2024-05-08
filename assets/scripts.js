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
