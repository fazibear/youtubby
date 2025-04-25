let Youtubby = function(){
  let postMessage = function(msg) {
    window.ipc.postMessage(JSON.stringify(msg));
  };

  let metadata = function() {
    meta = navigator.mediaSession.metadata
    data = {}
    for( key in meta) {
      data[key] = meta[key]
    }
    return data
  }

  let attachVideoEvents = function(video) {
    Object.entries({
      //"abort": (event) => postMessage({type: event.type}),
      //"canplay": (event) => postMessage({type: event.type}),
      //"canplaythrough": (event) => postMessage({type: event.type}),
      "durationchange": (event) => postMessage({type: event.type, duration: document.getElementById('progress-bar').max}),
      "emptied": (event) => postMessage({type: event.type}),
      "ended": (event) => postMessage({type: event.type}),
      //"error": (event) => postMessage({type: event.type}),
      //"loadeddata": (event) => postMessage({type: event.type}),
      "loadedmetadata": (event) => postMessage({type: event.type, metadata: metadata()}),
      //"loadstart": (event) => postMessage({type: event.type}),
      "pause": (event) => postMessage({type: event.type}),
      "play": (event) => postMessage({type: event.type}),
      //"playing": (event) => postMessage({type: event.type}),
      //"progress": (event) => postMessage({type: event.type}),
      //"ratechange": (event) => postMessage({type: event.type}),
      "seeked": (event) => postMessage({type: event.type}),
      //"seeking": (event) => postMessage({type: event.type}),
      //"stalled": (event) => postMessage({type: event.type}),
      //"suspend": (event) => postMessage({type: event.type}),
      "timeupdate": (event) => postMessage({type: event.type, time: document.getElementById('progress-bar').value}),
      //"volumechange": (event) => postMessage({type: event.type}),
      "waiting": (event) => postMessage({type: event.type}),
    }).forEach(([event, callback]) => video.addEventListener(event, callback));
  }

  let attachMetaDataUpdate = function() {
    const originalFetch = window.fetch;
    window.fetch = async (request, config) => {
      const response = await originalFetch(request, config);
      if(response.url.match(/next/) && response.ok === true) {
        setTimeout(function() {
          postMessage(
            {type: "metadataupdate", metadata: metadata()}
          );
        }, 100);
      }
      return response;
    };
  }

  var findAndAttachVideoEvents = function() {
    video = document.querySelector("video");
    if (!video){
      return setTimeout(findAndAttachVideoEvents, 250);
    }else{
      attachVideoEvents(video);
      video.parentNode.addEventListener('DOMNodeInserted', function(e) {
        attachVideoEvents(e.relatedNode.childNodes[0])
      });
    };
  }

  var loadCustomCSS = function() {
    var cssStyle = document.createElement('style');
    cssStyle.type = 'text/css';
    var rules = document.createTextNode(YoutubbyCustomCSS);
    cssStyle.appendChild(rules);
    document.querySelector('head').appendChild(cssStyle);
  }

  window.addEventListener("load", function() {
    loadCustomCSS();
    findAndAttachVideoEvents();
    attachMetaDataUpdate();
  });

  let playSomething = () => {
    let xpath = "//span[text()='Play all']";
    let play_all = document
      .evaluate(xpath, document, null, XPathResult.FIRST_ORDERED_NODE_TYPE, null)
      .singleNodeValue
      .parentNode
      .parentNode

    if(play_all) {
      play_all.click()
    } else {
      let buttons = document.getElementsByClassName("ytmusic-play-button-renderer");
      let rand = Math.floor(Math.random() * buttons.length);
      buttons[rand].click();
    }
  }

  let prevButtonClick = () => {
    document.querySelector(".previous-button").click();
  }

  let nextButtonClick = () => {
    document.querySelector(".next-button").click();
  }

  let playPauseButtonClick = () => {// #top-player-bar
    document.getElementById('play-pause-button').click();
  }

  let playPause = () => {
    if(document.getElementById('layout').getAttributeNames().includes('player-visible')){
      playPauseButtonClick();
    }else{
      playSomething();
    }
  }

  let switchToAudio = () => {
    document.getElementsByClassName("song-button")[0].click();
  }

  return {
    playPause: playPause,
    prev: prevButtonClick,
    next: nextButtonClick,
    switchToAudio: switchToAudio
  }
}();
