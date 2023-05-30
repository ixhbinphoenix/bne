//Yeah this is total Chat-GPT magic here

import { dir } from "console";

interface Options {
  direction: "left" | "right";
  renew?: boolean;
}

export function onSwipe(query: string, { direction, renew = false }: Options, callback: Function) {
  let touchStartX = 0;
  let touchEndX = 0;
  let touchStartY = 0;
  let touchEndY = 0;
  let touchStartTime = 0;

  const destination = document.querySelector(query);

  destination?.addEventListener("touchstart", handleTouchStart, { passive: true, once: true });
  destination?.addEventListener("touchend", handleTouchEnd, { passive: true, once: true });

  function handleTouchStart(event: Event) {
    if (event instanceof TouchEvent) {
      touchStartX = event.touches[0].clientX;
      touchStartY = event.touches[0].clientY;
      touchStartTime = new Date().getTime();
    }
  }

  function handleTouchEnd(event: Event) {
    if (event instanceof TouchEvent) {
      touchEndX = event.changedTouches[0].clientX;
      touchEndY = event.changedTouches[0].clientY;
      handleSwipeGesture();
    }
  }

  function handleSwipeGesture() {
    const swipeDistanceX = touchEndX - touchStartX;
    const swipeDistanceY = touchEndY - touchStartY;
    const swipeDuration = new Date().getTime() - touchStartTime;
    const swipeThreshold = 100; // Schwellenwert für die minimale Wischstrecke in Pixeln
    const maxSwipeDuration = 500; // Maximale Dauer der Wischgeste in Millisekunden
    const verticalThreshold = 50; // Schwellenwert für die maximale vertikale Bewegung in Pixeln
    const minSwipeSpeed = 0.5; // Minimale Geschwindigkeit der Wischgeste in Pixeln pro Millisekunde

    if (
      Math.abs(swipeDistanceX) > swipeThreshold &&
      Math.abs(swipeDistanceY) < verticalThreshold &&
      swipeDuration < maxSwipeDuration &&
      Math.abs(swipeDistanceX) / swipeDuration > minSwipeSpeed
    ) {
      if (swipeDistanceX > 0 && direction == "right") {
        console.info("right");
        callback();
      } else if (swipeDistanceX < 0 && direction == "left") {
        console.info("left");
        callback();
      } else if (
        (swipeDistanceX < 0 && direction == "right" && renew) ||
        (swipeDistanceX > 0 && direction == "left" && renew)
      ) {
        console.info("restart");
        destination?.addEventListener("touchstart", handleTouchStart, { passive: true, once: true });
        destination?.addEventListener("touchend", handleTouchEnd, { passive: true, once: true });
      }
    } else {
      destination?.addEventListener("touchstart", handleTouchStart, { passive: true, once: true });
      destination?.addEventListener("touchend", handleTouchEnd, { passive: true, once: true });
    }
  }
}
