import { Component, OnInit } from '@angular/core';
import { AppMainComponent } from '../app-main/app.main.component';

@Component({
  selector: 'app-menu',
  templateUrl: './menu.component.html',
})
export class MenuComponent implements OnInit {
  model!: any[];

  constructor(public appMain: AppMainComponent) {}

  ngOnInit(): void {
    this.model = [
      {
        label: 'Home',
        icon: 'start.svg',
        routerLink: ['/Home'],
      },
      {
        label: 'Start as Player',
        icon: 'start player.svg',
        routerLink: ['/Start-Player'],
      },
      {
        label: 'Start as Researcher',
        icon: 'start researcher.svg',
        routerLink: ['/Start-Researcher'],
      },
      {
        label: 'Game',
        icon: 'game.svg',
        routerLink: ['/Game'],
      },
      {
        label: 'Round Setup',
        icon: 'round setup.svg',
        routerLink: ['/Setup'],
      },
      {
        label: 'Round Report',
        icon: 'round report.svg',
        routerLink: ['/Round-Report'],
      },
    ];
  }
}
