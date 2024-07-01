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
      "pause": (event) => postMessage({type: event.type}),
      "play": (event) =>  postMessage({type: event.type}),
      "seeked": (event) => postMessage({type: event.type}),
      "durationchange": (event) => postMessage({type: event.type, duration: document.getElementById('progress-bar').max}),
      "timeupdate": (event) => postMessage({type: event.type, time: document.getElementById('progress-bar').value}),
      "emptied": (event) => {
        postMessage({type: event.type})
      },
      //"complete": (event) => postMessage({type: event.type}),
      "error": (event) => postMessage({type: event.type}),
      //"ended": (event) => postMessage({type: event.type}),
      //"loadedmetadata": (event) => postMessage({type: event.type, duration: progressBar.ariaValueMax})
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
      return setTimeout(init, 250);
    }else{
      attachVideoEvents(video);
      video.parentNode.addEventListener('DOMNodeInserted', function(e) {
        attachVideoEvents(e.relatedNode.childNodes[0])
      });
    };
  }

  let init = function() {
    findAndAttachVideoEvents();
    attachMetaDataUpdate();
  };

  window.addEventListener("load", init);

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

  let playPause = () => {// #top-player-bar
    document.getElementById('play-pause-button').click();
  }

  let playPauseClick = () => {
    if(document.getElementById('layout').getAttributeNames().includes('player-visible')){
      playPause();
    }else{
      playSomething();
    }
  }

  return {
    playPauseClick: playPauseClick
  }
}();

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
