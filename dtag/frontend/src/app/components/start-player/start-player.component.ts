import { Component, OnInit } from '@angular/core';
import { MenuItem } from 'primeng/api';

@Component({
  selector: 'app-start-player',
  templateUrl: './start-player.component.html',
  styleUrls: ['./start-player.component.scss'],
})
export class StartPlayerComponent implements OnInit {
  constructor() {}

  itemsBreadcumb: MenuItem[] = [
    { icon: 'pi pi-home', routerLink: '/' },
    { label: 'DTag' },
    { label: 'Start as Player' },
  ];

  ngOnInit(): void {}
}
