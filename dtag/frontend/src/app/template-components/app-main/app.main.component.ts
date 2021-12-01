import { Component } from '@angular/core';
import { PrimeNGConfig } from 'primeng/api';
import { AppComponent } from '../../app.component';
import { GameComponent } from '../../components/game/game.component';
import { HomeComponent } from '../../components/home/home.component';
import { RoundSetupComponent } from '../../components/round-setup/round-setup.component';
import { StartPlayerComponent } from '../../components/start-player/start-player.component';
import { StartResearcherComponent } from '../../components/start-researcher/start-researcher.component';
import { MenuService } from '../menu/menu.service';

@Component({
  selector: 'app-app.main',
  templateUrl: './app.main.component.html',
})
export class AppMainComponent {
  sidebarStatic!: boolean;
  sidebarActive = false;
  staticMenuMobileActive!: boolean;
  menuClick!: boolean;
  topbarItemClick!: boolean;
  activeTopbarItem: any;
  topbarMenuActive!: boolean;
  searchClick = false;
  search = false;
  rightPanelClick!: boolean;
  rightPanelActive!: boolean;
  configActive!: boolean;
  configClick!: boolean;
  menuHoverActive = false;
  title = '';

  constructor(
    private menuService: MenuService,
    private primengConfig: PrimeNGConfig,
    public app: AppComponent,
  ) {}

  onLayoutClick(): void {
    if (!this.topbarItemClick) {
      this.activeTopbarItem = null;
      this.topbarMenuActive = false;
    }

    if (!this.menuClick && (this.isHorizontal() || this.isSlim())) {
      this.menuService.reset();
    }

    if (this.configActive && !this.configClick) {
      this.configActive = false;
    }

    if (!this.rightPanelClick) {
      this.rightPanelActive = false;
    }

    if (!this.menuClick) {
      if (this.staticMenuMobileActive) {
        this.staticMenuMobileActive = false;
      }

      this.menuHoverActive = false;
      this.unblockBodyScroll();
    }

    if (!this.searchClick) {
      this.search = false;
    }

    this.searchClick = false;
    this.configClick = false;
    this.topbarItemClick = false;
    this.menuClick = false;
    this.rightPanelClick = false;
  }

  onMenuButtonClick(event: Event): void {
    this.menuClick = true;
    this.topbarMenuActive = false;
    this.rightPanelActive = false;

    if (this.isMobile()) {
      this.staticMenuMobileActive = !this.staticMenuMobileActive;
      if (this.staticMenuMobileActive) {
        this.blockBodyScroll();
      } else {
        this.unblockBodyScroll();
      }
    }

    event.preventDefault();
  }

  onTopbarItemClick(event: Event, item: Element): void {
    this.topbarItemClick = true;

    if (this.activeTopbarItem === item) {
      this.activeTopbarItem = null;
    } else {
      this.activeTopbarItem = item;
    }

    if (item.className === 'topbar-item search-item') {
      this.search = !this.search;
      this.searchClick = !this.searchClick;
    }

    event.preventDefault();
  }

  onRightPanelClick(event: Event): void {
    this.rightPanelClick = true;
    this.rightPanelActive = !this.rightPanelActive;

    this.staticMenuMobileActive = false;

    event.preventDefault();
  }

  onRippleChange(event: any): void {
    this.app.ripple = event.checked;
    this.primengConfig.ripple = event.checked;
  }

  onConfigClick(): void {
    this.configClick = true;
  }

  onSidebarClick(): void {
    this.menuClick = true;
  }

  onToggleMenu(event: Event): void {
    this.menuClick = true;
    this.sidebarStatic = !this.sidebarStatic;

    event.preventDefault();
  }

  onSidebarMouseOver(): void {
    if (this.app.menuMode === 'sidebar' && !this.sidebarStatic) {
      this.sidebarActive = !this.isMobile();
    }
  }

  onSidebarMouseLeave(): void {
    if (this.app.menuMode === 'sidebar' && !this.sidebarStatic) {
      setTimeout(() => {
        this.sidebarActive = false;
      }, 250);
    }
  }

  isSlim(): boolean {
    return this.app.menuMode === 'slim';
  }

  isHorizontal(): boolean {
    return this.app.menuMode === 'horizontal';
  }

  isDesktop(): boolean {
    return window.innerWidth > 991;
  }

  isMobile(): boolean {
    return window.innerWidth <= 991;
  }

  blockBodyScroll(): void {
    if (document.body.classList) {
      document.body.classList.add('blocked-scroll');
    } else {
      document.body.className += ' blocked-scroll';
    }
  }

  unblockBodyScroll(): void {
    if (document.body.classList) {
      document.body.classList.remove('blocked-scroll');
    } else {
      document.body.className = document.body.className.replace(
        new RegExp(
          '(^|\\b)' + 'blocked-scroll'.split(' ').join('|') + '(\\b|$)',
          'gi',
        ),
        ' ',
      );
    }
  }

  onRouterOutletActivate(component: any): void {
    if (component instanceof GameComponent) {
      this.title = 'Game';
    } else if (component instanceof HomeComponent) {
      this.title = 'DTag';
    } else if (component instanceof StartPlayerComponent) {
      this.title = 'Start as Player';
    } else if (component instanceof StartResearcherComponent) {
      this.title = 'Start as Researcher';
    } else if (component instanceof RoundSetupComponent) {
      this.title = 'Round Setup';
    }
  }
}
