import { Component, OnInit } from '@angular/core';
import { AppMainComponent } from '../app-main/app.main.component';

@Component({
  selector: 'app-menu',
  templateUrl: './menu.component.html',
  styleUrls: ['./menu.component.scss'],
})
export class MenuComponent implements OnInit {
  model!: any[];

  constructor(public appMain: AppMainComponent) {}

  ngOnInit(): void {
    this.model = [
      {
        label: 'Start',
        icon: 'start.svg',
        routerLink: ['/Start'],
      },
      {
        label: 'Game',
        icon: 'game.svg',
        routerLink: ['/Game'],
      },
      {
        label: 'Round Setup',
        icon: 'round setup.svg',
        routerLink: ['/Round-Setup'],
      },
      {
        label: 'Round Report',
        icon: 'round report.svg',
        routerLink: ['/Round-Report'],
      },
    ];
  }
}
