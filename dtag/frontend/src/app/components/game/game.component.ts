import { Component, OnInit, ViewChild } from '@angular/core';
import Cropper from 'cropperjs';
import { MenuItem } from 'primeng/api';
import SwiperCore, {
  EffectFade,
  Navigation,
  Pagination,
  SwiperOptions,
} from 'swiper';
import { SwiperComponent } from 'swiper/angular';

// install Swiper modules
SwiperCore.use([Navigation, Pagination, EffectFade]);

@Component({
  selector: 'app-icentivized-aitraining',
  templateUrl: './game.component.html',
  styleUrls: ['./game.component.scss'],
})
export class GameComponent implements OnInit {
  @ViewChild('swiper', { static: false }) swiper?: SwiperComponent;

  itemsBreadcumb: MenuItem[] = [
    { icon: 'pi pi-home', routerLink: '/' },
    { label: 'DTag' },
    { label: 'Game' },
  ];
  configSwiper: SwiperOptions = {
    slidesPerView: 1,
    effect: 'fade',
    centeredSlides: true,
    centeredSlidesBounds: true,
    spaceBetween: 1000,
    roundLengths: true,
    resistance: false,
    fadeEffect: {
      // Add this
      crossFade: true,
    },
    navigation: true,
    // pagination: { clickabl },
  };
  croppers: { [key: number]: Cropper | null } = {};
  showSample = false;
  showIncreaseBet = false;
  showDIDDetails = false;
  totalBet = 0;
  showDisclose = false;

  constructor() {}

  ngOnInit(): void {}

  private static createCropper(image: HTMLImageElement) {
    return new Cropper(image, {
      viewMode: 1,
    });
  }

  showCropper() {
    if (this.swiper) {
      const index = this.swiper.swiperRef.realIndex;
      const slide = this.swiper.swiperRef.slides[index];
      const image = slide.querySelector('img')!;
      if (this.croppers[index] === undefined || this.croppers[index] === null) {
        this.croppers[index] = GameComponent.createCropper(image);
      }
    }
  }

  removeCropper() {
    if (this.swiper) {
      const index = this.swiper.swiperRef.realIndex;
      this.croppers[index]?.destroy();
      this.croppers[index] = null;
      // this.croppers[index]?.reset();
      // this.croppers[index]?.clear();
    }
  }

  btnSampleClick() {
    this.showSample = true;
  }

  hideSample() {
    this.showSample = false;
  }

  hideIncreaseBet() {
    this.showIncreaseBet = false;
  }

  btnIncreaseBetClick() {
    this.showIncreaseBet = true;
  }

  hideDIDDetails() {
    this.showDIDDetails = false;
  }

  confirmIncreaseBet() {
    const pIncrement = document.querySelector<HTMLInputElement>('#pIncrement')!;
    const increment = parseInt(pIncrement.textContent?.trim() ?? '0');
    this.totalBet += increment;
    this.showIncreaseBet = false;
    pIncrement.textContent = '0';
  }

  submitDisclose() {
    this.showDisclose = false;
    // Fetch
    this.showDIDDetails = true;
  }
}
