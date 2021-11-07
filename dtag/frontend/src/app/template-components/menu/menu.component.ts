import { Component, OnInit } from '@angular/core';
import {AppMainComponent} from "../app-main/app.main.component";

@Component({
  selector: 'app-menu',
  templateUrl: './menu.component.html',
  styleUrls: ['./menu.component.scss']
})
export class MenuComponent implements OnInit {
  model!: any[];

  constructor(public appMain: AppMainComponent) {}

  ngOnInit(): void {
    this.model = [
      {
        label: 'Home',
        // icon: 'pi pi-star-o',
        icon: 'home.svg',
        // icon: '',
        routerLink: ['/'],
      },
      {
        label: 'Farm',
        // icon: 'pi pi-star-o',
        icon: 'farm/farm.svg',
        routerLink: ['/crops'],
        items: [
          {
            label: 'Seeds Management',
            icon: 'farm/seeds-management.svg',
            routerLink: ['/seeds'],
          },
          {
            label: 'Plots Management',
            icon: 'farm/plot-management.svg',
            routerLink: ['/plots'],
          },
          {
            label: 'Crops Planning',
            icon: 'farm/crop-planning.svg',
            routerLink: ['/crops'],
          },
          // {
          //   label: 'Harvest Weighting',
          //   icon: 'farm/harvest-weighting.svg',
          //   routerLink: ['/plots'],
          // },
          {
            label: 'Task Automation',
            icon: 'farm/task-automation.svg',
            routerLink: ['/task-automation'],
          },
          {
            label: 'DT Tracking',
            icon: 'farm/dt-tracking.svg',
            routerLink: ['/dttracking'],
          },
        ],
      },
      {
        label: 'IoT Management',
        icon: 'iot/iot-management.svg',
        routerLink: ['/devices'],
        items: [
          {
            label: 'Static Devices',
            icon: 'iot/static.svg',
            routerLink: ['/devices-static'],
          },
          {
            label: 'Mobile Devices',
            icon: 'iot/mobile.svg',
            routerLink: ['device-selection'],
          },
        ],
      },
    ];
  }
}
