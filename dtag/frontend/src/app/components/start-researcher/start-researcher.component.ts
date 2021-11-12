import { Component, OnInit } from '@angular/core';
import { MenuItem } from 'primeng/api';

@Component({
  selector: 'app-start-researcher',
  templateUrl: './start-researcher.component.html',
  styleUrls: ['./start-researcher.component.scss'],
})
export class StartResearcherComponent implements OnInit {
  constructor() {}

  itemsBreadcumb: MenuItem[] = [
    { icon: 'pi pi-home', routerLink: '/' },
    { label: 'DTag' },
    { label: 'Start as Researcher' },
  ];
  showReportPreview = false;
  showCreateDID = false;
  showCreateVerifiableCredential = false;
  showDIDDetails = false;
  showSignIn = false;

  ngOnInit(): void {}

  createDID() {
    this.showCreateDID = false;
    this.showCreateVerifiableCredential = true;
  }

  showResponse($event: any) {
    console.log($event);
  }

  verifyVerifiablePresentation() {
    this.showCreateVerifiableCredential = false;
    this.showDIDDetails = true;
  }

  signIn() {}
}
