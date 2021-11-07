import {Component, OnDestroy, OnInit} from '@angular/core';
import {MenuItem} from "primeng/api";
import {Subscription} from "rxjs";
import {AppComponent} from "../../app.component";
import {AppMainComponent} from "../app-main/app.main.component";

@Component({
  selector: 'app-topbar',
  templateUrl: './topbar.component.html',
  styleUrls: ['./topbar.component.scss']
})
export class TopbarComponent implements OnDestroy {
  subscription!: Subscription;

  items!: MenuItem[];

  constructor(public app: AppComponent, public appMain: AppMainComponent) {}

  ngOnDestroy(): void {
    this.subscription?.unsubscribe();
  }
}
