import { Component, OnInit } from '@angular/core';
import { MenuItem } from 'primeng/api';
import { Clipboard } from '@angular/cdk/clipboard';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss'],
})
export class HomeComponent implements OnInit {
  itemsBreadcumb: MenuItem[] = [
    { icon: 'pi pi-home', routerLink: '/' },
    {
      label: 'DTag',
    },
  ];
  showDonate = false;
  showListDonors = false;
  donationAddress =
    'b1182c08a923b237faefa09c915ed8e1abfd1fb7f747bce839a50191fce0256a';

  constructor(private clipboard: Clipboard) {}

  ngOnInit(): void {}

  btnShowDonorsOnClick() {
    this.showDonate = false;
    this.showListDonors = true;
  }

  copyDonationAddress() {
    this.clipboard.copy(this.donationAddress);
  }
}
