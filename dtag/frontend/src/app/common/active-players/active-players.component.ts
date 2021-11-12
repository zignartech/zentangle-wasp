import { Component, Input, OnInit } from '@angular/core';

@Component({
  selector: 'app-active-players',
  templateUrl: './active-players.component.html',
  styleUrls: ['./active-players.component.scss'],
})
export class ActivePlayersComponent implements OnInit {
  constructor() {}

  @Input('maxHeight') maxHeight: string = '50%';

  ngOnInit(): void {}
}
