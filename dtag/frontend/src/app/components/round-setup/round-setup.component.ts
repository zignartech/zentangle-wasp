import { Component, OnInit, ViewChild } from '@angular/core';
import { NgxDropzoneComponent } from 'ngx-dropzone';
import { MenuItem } from 'primeng/api';

@Component({
  selector: 'app-round-setup',
  templateUrl: './round-setup.component.html',
  styleUrls: ['./round-setup.component.scss'],
})
export class RoundSetupComponent implements OnInit {
  @ViewChild('drop') dropzone!: NgxDropzoneComponent;

  modeImages = 'local-images';
  itemsBreadcumb: MenuItem[] = [
    { icon: 'pi pi-home', routerLink: '/' },
    { label: 'DTag' },
    { label: 'Setup' },
  ];

  constructor() {}

  ngOnInit(): void {}
}
