import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ActivePlayersComponent } from './active-players.component';

describe('ActivePlayersComponent', () => {
  let component: ActivePlayersComponent;
  let fixture: ComponentFixture<ActivePlayersComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ActivePlayersComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(ActivePlayersComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
